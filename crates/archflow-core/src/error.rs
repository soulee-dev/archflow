use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArchflowError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Layout error: {0}")]
    LayoutError(String),

    #[error("Render error: {0}")]
    RenderError(String),
}

impl From<serde_json::Error> for ArchflowError {
    fn from(e: serde_json::Error) -> Self {
        ArchflowError::InvalidJson(e.to_string())
    }
}
