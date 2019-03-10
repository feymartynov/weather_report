/// Shared structures for parsing API responses.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
