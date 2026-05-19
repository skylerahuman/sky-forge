use serde::{Deserialize, Serialize};

use crate::{CommandRecord, FileChange, WorkspacePath};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    ReadFile {
        path: WorkspacePath,
    },
    Search {
        pattern: String,
        path: Option<WorkspacePath>,
    },
    EditFile {
        path: WorkspacePath,
        patch: String,
    },
    Execute {
        command: String,
    },
    AskUser {
        question: String,
    },
    Finish {
        summary: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActionResult {
    Text(String),
    FileContent {
        path: WorkspacePath,
        content: String,
    },
    Diff(FileChange),
    Command(CommandRecord),
    Error(String),
}
