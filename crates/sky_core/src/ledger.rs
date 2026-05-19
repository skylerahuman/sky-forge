use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Action, ActionResult, Instruction, RunId, TurnId};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateLedger {
    pub run_id: RunId,
    pub instruction: Instruction,
    pub entries: Vec<LedgerEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub turn_id: TurnId,
    pub timestamp: DateTime<Utc>,
    pub kind: LedgerEntryKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerEntryKind {
    UserInstruction(String),
    ProviderOutput(String),
    Action(Action),
    ActionResult(ActionResult),
    Error(String),
    Question(String),
    Summary(String),
    Telemetry {
        tokens: u64,
        elapsed_ms: u64,
        estimated_cost: Option<f64>,
    },
}

impl StateLedger {
    pub fn new(instruction: Instruction) -> Self {
        Self { run_id: RunId::generate(), instruction, entries: Vec::new() }
    }

    pub fn append(&mut self, turn_id: TurnId, kind: LedgerEntryKind) {
        self.entries
            .push(LedgerEntry { turn_id, timestamp: Utc::now(), kind });
    }

    pub fn compact_text(&self) -> String {
        let mut out = format!("Instruction:\n{}\n", self.instruction.as_str());
        for entry in &self.entries {
            out.push_str(&format!("\n[{}] {:?}", entry.turn_id, entry.kind));
        }
        out
    }
}
