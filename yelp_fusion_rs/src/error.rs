use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::{error, fmt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[expect(clippy::module_name_repetitions)]
pub struct ApiError {
    pub code: String,
    pub description: String,
}

#[derive(Debug)]
pub enum Error {
    /// Error occurred while using the `reqwest` library.
    ReqwestError(reqwest::Error),

    /// Error occurred while using the `serde` library.
    SerdeJsonError(serde_json::Error),

    /// An API request returned with a failed status code.
    RequestFailed {
        error_response: ApiErrorResponse,
        status_code: StatusCode,
    },
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReqwestError(e) => fmt::Display::fmt(e, f),
            Self::SerdeJsonError(e) => fmt::Display::fmt(e, f),
            Self::RequestFailed {
                error_response,
                status_code,
            } => {
                write!(
                    f,
                    "[{}] {}: {}",
                    status_code, error_response.error.code, error_response.error.description
                )
            }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}
