use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use sky_core::ProviderResponse;
use sky_engine::{Provider, ProviderRequest};

#[derive(Clone, Debug)]
pub struct AnthropicProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
}

impl AnthropicProvider {
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            model: model.into(),
        })
    }

    pub fn from_env() -> anyhow::Result<Option<Self>> {
        let Ok(base_url) = std::env::var("ANTHROPIC_BASE_URL") else {
            return Ok(None);
        };
        let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") else {
            return Ok(None);
        };
        let model =
            std::env::var("ANTHROPIC_MODEL").unwrap_or_else(|_| "claude-sonnet-4-5".to_string());
        Ok(Some(Self::new(base_url, api_key, model)?))
    }

    fn messages_url(&self) -> String {
        if self.base_url.ends_with("/v1/messages") {
            self.base_url.clone()
        } else if self.base_url.ends_with("/v1") {
            format!("{}/messages", self.base_url)
        } else {
            format!("{}/v1/messages", self.base_url)
        }
    }
}

#[async_trait::async_trait]
impl Provider for AnthropicProvider {
    async fn respond(&self, request: ProviderRequest<'_>) -> anyhow::Result<ProviderResponse> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 1024,
            system: request.system_prompt.to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: request.session.ledger().instruction.as_str().to_string(),
            }],
        };

        let response = self
            .client
            .post(self.messages_url())
            .header("x-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            anyhow::bail!("anthropic provider returned {status}: {body}");
        }

        let parsed: AnthropicResponse = serde_json::from_str(&body)?;
        let raw = parsed
            .content
            .into_iter()
            .filter(|part| part.kind == "text")
            .filter_map(|part| part.text)
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ProviderResponse { raw })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sky_engine::{Provider, default_system_prompt};
    use sky_ledger::LedgerSession;

    #[test]
    fn builds_messages_url_from_base_url() {
        let provider = AnthropicProvider::new("http://localhost:3456", "key", "model").unwrap();
        assert_eq!(provider.messages_url(), "http://localhost:3456/v1/messages");

        let provider = AnthropicProvider::new("http://localhost:3456/v1", "key", "model").unwrap();
        assert_eq!(provider.messages_url(), "http://localhost:3456/v1/messages");
    }

    #[test]
    fn serializes_system_prompt() {
        let request = AnthropicRequest {
            model: "model".to_string(),
            max_tokens: 1024,
            system: default_system_prompt().to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: "hello".to_string(),
            }],
        };
        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["system"], default_system_prompt());
        assert_eq!(json["messages"][0]["content"], "hello");
    }

    #[tokio::test]
    #[ignore]
    async fn live_anthropic_provider_responds() {
        let provider = AnthropicProvider::from_env()
            .unwrap()
            .expect("ANTHROPIC_BASE_URL and ANTHROPIC_API_KEY must be set");
        let session = LedgerSession::new("Reply with exactly: sky live provider ok");
        let response = provider
            .respond(ProviderRequest { system_prompt: default_system_prompt(), session: &session })
            .await
            .unwrap();
        assert!(!response.raw.trim().is_empty());
        println!("{}", response.raw.trim());
    }
}
