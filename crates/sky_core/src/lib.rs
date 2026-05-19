mod action;
mod approval;
mod command;
mod error;
mod file_change;
mod ids;
mod instruction;
mod ledger;
mod paths;
mod provider;
mod tool;

pub use action::*;
pub use approval::*;
pub use command::*;
pub use error::*;
pub use file_change::*;
pub use ids::*;
pub use instruction::*;
pub use ledger::*;
pub use paths::*;
pub use provider::*;
pub use tool::*;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn ledger_appends_entries_in_order() {
        let mut ledger = StateLedger::new(Instruction::new("do the thing"));
        let turn = TurnId::generate();
        ledger.append(
            turn,
            LedgerEntryKind::ProviderOutput("thinking".to_string()),
        );
        ledger.append(turn, LedgerEntryKind::Summary("done".to_string()));
        assert_eq!(ledger.entries.len(), 2);
    }

    #[test]
    fn workspace_path_rejects_escape_attempts() {
        assert!(WorkspacePath::new("src/lib.rs").is_ok());
        assert!(WorkspacePath::new("../secret").is_err());
        assert!(WorkspacePath::new("/etc/passwd").is_err());
    }

    #[test]
    fn command_records_success_and_replay_mode() {
        let record = CommandRecord {
            command: "cargo test".to_string(),
            cwd: WorkspacePath::new(".").unwrap(),
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
            replay_mode: ReplayMode::RequiresApproval,
        };
        assert!(record.succeeded());
        assert_eq!(record.replay_mode, ReplayMode::RequiresApproval);
    }

    #[test]
    fn actions_are_strict_internal_commands() {
        let action = Action::Search { pattern: "TODO".to_string(), path: None };
        assert!(matches!(action, Action::Search { .. }));
    }
}
