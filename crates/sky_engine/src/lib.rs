use sky_core::{Action, ActionResult, ProviderResponse, TurnId, WorkspacePath};
use sky_ledger::LedgerSession;
use sky_workspace::Workspace;

pub fn default_system_prompt() -> &'static str {
    include_str!("../prompts/default_system.md").trim()
}

#[derive(Clone, Copy, Debug)]
pub struct ProviderRequest<'a> {
    pub system_prompt: &'a str,
    pub session: &'a LedgerSession,
}

#[derive(Clone, Debug)]
pub struct OneTurnEngine<P = FakeProvider> {
    provider: P,
    workspace: Workspace,
}

#[derive(Clone, Debug, Default)]
pub struct FakeProvider;

impl OneTurnEngine<FakeProvider> {
    pub fn fake() -> Self {
        Self::new(FakeProvider)
    }
}

impl<P: Provider> OneTurnEngine<P> {
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            workspace: Workspace::current().expect("current directory must be a workspace"),
        }
    }

    pub fn with_workspace(provider: P, workspace: Workspace) -> Self {
        Self { provider, workspace }
    }

    pub async fn run(&self, instruction: impl Into<String>) -> anyhow::Result<LedgerSession> {
        let mut session = LedgerSession::new(instruction.into());
        let turn = TurnId::generate();

        session.append_user_instruction(turn);
        let response = self
            .provider
            .respond(ProviderRequest { system_prompt: default_system_prompt(), session: &session })
            .await?;
        session.append_provider_response(turn, &response);

        let action = match parse_action(&response) {
            Ok(action) => action,
            Err(error) => {
                session.append_error(turn, error.to_string());
                return Ok(session);
            }
        };
        session.append_action(turn, action.clone());

        let result = execute_action(&self.workspace, &action).await;
        session.append_action_result(turn, result);

        if let Action::Finish { summary } = action {
            session.append_summary(turn, summary);
        }

        Ok(session)
    }
}

#[async_trait::async_trait]
pub trait Provider {
    async fn respond(&self, request: ProviderRequest<'_>) -> anyhow::Result<ProviderResponse>;
}

#[async_trait::async_trait]
impl Provider for FakeProvider {
    async fn respond(&self, request: ProviderRequest<'_>) -> anyhow::Result<ProviderResponse> {
        let instruction = request.session.ledger().instruction.as_str();
        let raw = serde_json::json!({
            "action": "finish",
            "summary": format!("Received instruction: {instruction}")
        })
        .to_string();

        Ok(ProviderResponse { raw })
    }
}

pub fn parse_action(response: &ProviderResponse) -> anyhow::Result<Action> {
    if response.raw.contains("<function_calls>") || response.raw.contains("<invoke ") {
        anyhow::bail!("provider used unsupported function-call markup instead of Sky action JSON");
    }

    let value: serde_json::Value = serde_json::from_str(&response.raw)
        .map_err(|_| anyhow::anyhow!("provider response was not valid Sky action JSON"))?;
    let action = value
        .get("action")
        .and_then(|value| value.as_str())
        .unwrap_or_default();

    match action {
        "finish" => Ok(Action::Finish {
            summary: value
                .get("summary")
                .and_then(|value| value.as_str())
                .unwrap_or("Finished")
                .to_string(),
        }),
        "ask_user" => Ok(Action::AskUser {
            question: value
                .get("question")
                .and_then(|value| value.as_str())
                .unwrap_or("What should I do next?")
                .to_string(),
        }),
        "read_file" => Ok(Action::ReadFile {
            path: WorkspacePath::new(
                value
                    .get("path")
                    .and_then(|value| value.as_str())
                    .unwrap_or_default(),
            )?,
        }),
        other => anyhow::bail!("unsupported action: {other}"),
    }
}

pub async fn execute_action(workspace: &Workspace, action: &Action) -> ActionResult {
    match action {
        Action::Finish { summary } => ActionResult::Text(summary.clone()),
        Action::AskUser { question } => ActionResult::Text(question.clone()),
        Action::ReadFile { path } => workspace.read_file(path).await,
        other => ActionResult::Error(format!("not executable in one-turn demo: {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sky_core::LedgerEntryKind;

    use super::*;

    #[test]
    fn fake_engine_records_one_turn() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let session = rt.block_on(OneTurnEngine::fake().run("hello")).unwrap();
        assert_eq!(session.ledger().entries.len(), 5);
        assert!(matches!(
            session.ledger().entries[2].kind,
            LedgerEntryKind::Action(_)
        ));
    }

    #[test]
    fn default_system_prompt_sets_basic_persona() {
        let prompt = default_system_prompt();
        assert!(prompt.contains("You are Sky"));
        assert!(prompt.contains("local development environment"));
        assert!(prompt.contains("terminal-first coding assistant"));
        assert!(prompt.contains("Every response must be exactly one JSON object"));
        assert!(prompt.contains("finish"));
        assert!(prompt.contains("ask_user"));
        assert!(prompt.contains("read_file"));
        assert!(prompt.contains("function-call markup"));
        assert!(prompt.contains("Never claim you read a file"));
    }

    #[test]
    fn parses_finish_action() {
        let response =
            ProviderResponse { raw: r#"{"action":"finish","summary":"done"}"#.to_string() };
        assert_eq!(
            parse_action(&response).unwrap(),
            Action::Finish { summary: "done".to_string() }
        );
    }

    #[test]
    fn rejects_plain_text_as_action() {
        let response = ProviderResponse { raw: "hello".to_string() };
        assert!(parse_action(&response).is_err());
    }

    #[test]
    fn rejects_hallucinated_function_call_markup() {
        let response = ProviderResponse {
            raw: r#"Reading now.<function_calls><invoke name="Read"></invoke></function_calls>"#
                .to_string(),
        };
        assert!(parse_action(&response).is_err());
    }

    #[test]
    fn parses_read_file_action() {
        let response = ProviderResponse {
            raw: r#"{"action":"read_file","path":"Cargo.toml"}"#.to_string(),
        };
        assert_eq!(
            parse_action(&response).unwrap(),
            Action::ReadFile { path: WorkspacePath::new("Cargo.toml").unwrap() }
        );
    }

    #[tokio::test]
    async fn engine_executes_read_file_action() {
        #[derive(Clone, Debug)]
        struct ReadProvider;

        #[async_trait::async_trait]
        impl Provider for ReadProvider {
            async fn respond(
                &self,
                _request: ProviderRequest<'_>,
            ) -> anyhow::Result<ProviderResponse> {
                Ok(ProviderResponse {
                    raw: r#"{"action":"read_file","path":"hello.txt"}"#.to_string(),
                })
            }
        }

        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("hello.txt"), "hello sky").unwrap();
        let workspace = Workspace::at(temp.path()).unwrap();
        let engine = OneTurnEngine::with_workspace(ReadProvider, workspace);
        let session = engine.run("read hello").await.unwrap();

        assert!(matches!(
            &session.ledger().entries[3].kind,
            LedgerEntryKind::ActionResult(ActionResult::FileContent { content, .. }) if content == "hello sky"
        ));
    }

    #[tokio::test]
    async fn engine_records_parse_error_without_losing_provider_output() {
        #[derive(Clone, Debug)]
        struct BadProvider;

        #[async_trait::async_trait]
        impl Provider for BadProvider {
            async fn respond(
                &self,
                _request: ProviderRequest<'_>,
            ) -> anyhow::Result<ProviderResponse> {
                Ok(ProviderResponse {
                    raw: r#"Reading now.<function_calls><invoke name="Read"></invoke></function_calls>"#
                        .to_string(),
                })
            }
        }

        let temp = tempfile::tempdir().unwrap();
        let workspace = Workspace::at(temp.path()).unwrap();
        let engine = OneTurnEngine::with_workspace(BadProvider, workspace);
        let session = engine.run("read hello").await.unwrap();

        assert!(matches!(
            session.ledger().entries[1].kind,
            LedgerEntryKind::ProviderOutput(_)
        ));
        assert!(matches!(
            session.ledger().entries[2].kind,
            LedgerEntryKind::Error(_)
        ));
    }
}
