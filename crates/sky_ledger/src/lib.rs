use sky_core::{
    Action, ActionResult, Instruction, LedgerEntryKind, ProviderResponse, StateLedger, TurnId,
};

#[derive(Clone, Debug, PartialEq)]
pub struct LedgerSession {
    ledger: StateLedger,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerBlock {
    pub title: String,
    pub body: String,
}

impl LedgerSession {
    pub fn new(instruction: impl Into<String>) -> Self {
        Self {
            ledger: StateLedger::new(Instruction::new(instruction.into())),
        }
    }

    pub fn ledger(&self) -> &StateLedger {
        &self.ledger
    }

    pub fn append_user_instruction(&mut self, turn_id: TurnId) {
        self.ledger.append(
            turn_id,
            LedgerEntryKind::UserInstruction(self.ledger.instruction.as_str().to_string()),
        );
    }

    pub fn append_provider_response(&mut self, turn_id: TurnId, response: &ProviderResponse) {
        self.ledger.append(
            turn_id,
            LedgerEntryKind::ProviderOutput(response.raw.clone()),
        );
    }

    pub fn append_action(&mut self, turn_id: TurnId, action: Action) {
        self.ledger.append(turn_id, LedgerEntryKind::Action(action));
    }

    pub fn append_action_result(&mut self, turn_id: TurnId, result: ActionResult) {
        self.ledger
            .append(turn_id, LedgerEntryKind::ActionResult(result));
    }

    pub fn append_summary(&mut self, turn_id: TurnId, summary: impl Into<String>) {
        self.ledger
            .append(turn_id, LedgerEntryKind::Summary(summary.into()));
    }

    pub fn append_error(&mut self, turn_id: TurnId, error: impl Into<String>) {
        self.ledger
            .append(turn_id, LedgerEntryKind::Error(error.into()));
    }

    pub fn blocks(&self) -> Vec<LedgerBlock> {
        let mut blocks = vec![LedgerBlock {
            title: "Instruction".to_string(),
            body: self.ledger.instruction.as_str().to_string(),
        }];

        blocks.extend(self.ledger.entries.iter().map(|entry| {
            let (title, body) = match &entry.kind {
                LedgerEntryKind::UserInstruction(value) => ("User Instruction", value.clone()),
                LedgerEntryKind::ProviderOutput(value) => ("Provider Output", value.clone()),
                LedgerEntryKind::Action(value) => ("Action", format!("{value:?}")),
                LedgerEntryKind::ActionResult(value) => ("Action Result", format!("{value:?}")),
                LedgerEntryKind::Error(value) => ("Error", value.clone()),
                LedgerEntryKind::Question(value) => ("Question", value.clone()),
                LedgerEntryKind::Summary(value) => ("Summary", value.clone()),
                LedgerEntryKind::Telemetry { tokens, elapsed_ms, estimated_cost } => (
                    "Telemetry",
                    format!(
                        "tokens={tokens} elapsed_ms={elapsed_ms} estimated_cost={estimated_cost:?}"
                    ),
                ),
            };

            LedgerBlock { title: title.to_string(), body }
        }));

        blocks
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sky_core::{Action, TurnId};

    use super::*;

    #[test]
    fn appends_instruction_and_action_blocks() {
        let mut session = LedgerSession::new("hello");
        let turn = TurnId::generate();

        session.append_user_instruction(turn);
        session.append_action(turn, Action::Finish { summary: "done".to_string() });

        let blocks = session.blocks();
        assert_eq!(blocks[0].title, "Instruction");
        assert_eq!(blocks[1].title, "User Instruction");
        assert_eq!(blocks[2].title, "Action");
    }
}
