use anyhow::anyhow;
use serde::{Deserialize, Serialize};

/// Response if the `Symbiosis` API cannot process the request body because of user input.
/// .e.g when some field which is provided by user is unsupported.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnprocessableEntity {
    code: u64,
    message: String,
    errors: Vec<EntityError>,
}

/// Describe the entity error specifically with `field` and `message`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityError {
    field: String,
    message: String,
}

/// Response if the `Symbiosis` API cannot process the request body internally.
/// .e.g insufficient liquidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequest {
    code: u64,
    message: String,
}

/// Symbiosis error in response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SymbiosisError {
    /// The data provided by user cannot be processed by Symbiosis.
    UnprocessableEntity(UnprocessableEntity),
    /// Symbiosis cannot sastify user request.
    BadRequest(BadRequest),
}

impl std::fmt::Display for SymbiosisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            SymbiosisError::UnprocessableEntity(err) => err.message.clone(),
            SymbiosisError::BadRequest(err) => err.message.clone(),
        };
        write!(f, "{}", message)
    }
}

impl std::fmt::Display for UnprocessableEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Display for BadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SymbiosisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SymbiosisError::UnprocessableEntity(err) => Some(err),
            SymbiosisError::BadRequest(err) => Some(err),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::error::Error for UnprocessableEntity {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::error::Error for BadRequest {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
