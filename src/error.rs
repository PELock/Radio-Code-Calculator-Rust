//! Errors returned by the Radio Code Calculator API interface
//!
//! Usage:
//!
//! ```ignore
//! if error == RadioErrors::SUCCESS { ... }
//! ```

use serde_json::Value;
use thiserror::Error;

/// Errors returned by [`crate::RadioCodeCalculator`] (license, HTTP/JSON, or API error payloads).
#[derive(Debug, Error)]
pub enum RadioCodeCalculatorError {
    /// @var int license key is invalid or expired (also when API key is missing)
    #[error("invalid license")]
    InvalidLicense,

    /// API returned `error != SUCCESS` (body includes the `error` field and optional details).
    #[error("API error: {0:?}")]
    ApiError(Value),

    /// Network failure, non-JSON response, or unexpected data.
    #[error("transport error: {0}")]
    Transport(String),
}

impl RadioCodeCalculatorError {
    /// Return the API `error` code when this is [`RadioCodeCalculatorError::ApiError`].
    pub fn api_error_code(&self) -> Option<i64> {
        match self {
            RadioCodeCalculatorError::ApiError(v) => v.get("error").and_then(|e| e.as_i64()),
            _ => None,
        }
    }
}

/// Errors returned by the Radio Code Calculator API interface
///
/// Usage:
///
/// if (error === RadioErrors.SUCCESS) { ... }
pub struct RadioErrors;

impl RadioErrors {
    /// @var int cannot connect to the Web API interface (network error)
    pub const ERROR_CONNECTION: i32 = -1;

    /// @var int successful request
    pub const SUCCESS: i32 = 0;

    /// @var int an error occurred while validating input data (invalid length, format etc.)
    pub const INVALID_INPUT: i32 = 1;

    /// @var int invalid Web API command (not supported)
    pub const INVALID_COMMAND: i32 = 2;

    /// @var int radio model is not supported by the calculator
    pub const INVALID_RADIO_MODEL: i32 = 3;

    /// @var int radio serial number is invalid (invalid format, not matching the expected regex pattern)
    pub const INVALID_SERIAL_LENGTH: i32 = 4;

    /// @var int radio serial number doesn't match the expected regular expression pattern
    pub const INVALID_SERIAL_PATTERN: i32 = 5;

    /// @var int radio serial number is not supported by the selected calculator
    pub const INVALID_SERIAL_NOT_SUPPORTED: i32 = 6;

    /// @var int extra data is invalid (invalid format, not matching the expected regex pattern)
    pub const INVALID_EXTRA_LENGTH: i32 = 7;

    /// @var int extra data doesn't match the expected regular expression pattern
    pub const INVALID_EXTRA_PATTERN: i32 = 8;

    /// @var int license key is invalid or expired
    pub const INVALID_LICENSE: i32 = 100;
}
