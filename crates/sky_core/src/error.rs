pub type Result<T> = std::result::Result<T, SkyCoreError>;

#[derive(Debug, thiserror::Error)]
pub enum SkyCoreError {
    #[error("unsafe path outside workspace or sandbox: {0}")]
    UnsafePath(String),
}
