use std::path::{Path, PathBuf};

use sky_core::{ActionResult, WorkspacePath};

#[derive(Clone, Debug)]
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn current() -> anyhow::Result<Self> {
        Self::at(std::env::current_dir()?)
    }

    pub fn at(root: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let root = root.into().canonicalize()?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub async fn read_file(&self, path: &WorkspacePath) -> ActionResult {
        match self.read_file_inner(path).await {
            Ok(content) => ActionResult::FileContent { path: path.clone(), content },
            Err(error) => ActionResult::Error(error.to_string()),
        }
    }

    async fn read_file_inner(&self, path: &WorkspacePath) -> anyhow::Result<String> {
        let absolute = self.root.join(path.as_path());
        let canonical = absolute.canonicalize()?;
        if !canonical.starts_with(&self.root) {
            anyhow::bail!("path escapes workspace: {path}");
        }
        Ok(tokio::fs::read_to_string(canonical).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[tokio::test]
    async fn reads_file_inside_workspace() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("hello.txt"), "hello sky").unwrap();
        let workspace = Workspace::at(temp.path()).unwrap();
        let path = WorkspacePath::new("hello.txt").unwrap();

        assert_eq!(
            workspace.read_file(&path).await,
            ActionResult::FileContent { path, content: "hello sky".to_string() }
        );
    }

    #[test]
    fn rejects_missing_workspace_root() {
        let temp = tempfile::tempdir().unwrap();
        let missing = temp.path().join("missing");
        assert!(Workspace::at(missing).is_err());
    }
}
