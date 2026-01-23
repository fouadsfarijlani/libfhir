use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FhirError {
    #[error("serialization error occured: {0}")]
    SerdeError(#[from] serde_json::Error),
}
