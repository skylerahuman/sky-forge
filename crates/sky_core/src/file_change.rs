use serde::{Deserialize, Serialize};

use crate::WorkspacePath;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FileChange {
    pub path: WorkspacePath,
    pub before_hash: String,
    pub after_hash: String,
    pub diff: String,
}
