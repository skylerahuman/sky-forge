use serde::{Deserialize, Serialize};

use crate::WorkspacePath;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandRecord {
    pub command: String,
    pub cwd: WorkspacePath,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub replay_mode: ReplayMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReplayMode {
    Replayable,
    SandboxOnly,
    RequiresApproval,
    Forbidden,
}

impl CommandRecord {
    pub fn succeeded(&self) -> bool {
        self.exit_code == 0
    }
}
