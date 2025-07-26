use thiserror::Error;

/// Errors that can occur when working with the Durgod K320 keyboard
#[derive(Error, Debug)]
pub enum DurgodError {
    /// HID API initialization failed
    #[error("Failed to initialize HID API: {0}")]
    HidInitError(String),

    /// Device not found or could not be opened
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// Device is not connected
    #[error("Device is not connected")]
    NotConnected,

    /// Communication error with the device
    #[error("Communication error: {0}")]
    CommunicationError(String),

    /// Invalid key code provided
    #[error("Invalid key code: {0}")]
    InvalidKeyCode(u8),

    /// Invalid HID report format
    #[error("Invalid HID report format: {0}")]
    InvalidReport(String),

    /// Device configuration error
    #[error("Device configuration error: {0}")]
    ConfigurationError(String),

    /// Permission error (usually requires sudo/admin)
    #[error("Permission denied - you may need to run with elevated privileges or set up udev rules")]
    PermissionDenied,

    /// Device busy or in use by another application
    #[error("Device is busy or in use by another application")]
    DeviceBusy,

    /// Timeout occurred during operation
    #[error("Operation timed out")]
    Timeout,

    /// Generic I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<hidapi::HidError> for DurgodError {
    fn from(error: hidapi::HidError) -> Self {
        match error {
            hidapi::HidError::HidApiError { message } => {
                if message.contains("Permission denied") || message.contains("Access denied") {
                    DurgodError::PermissionDenied
                } else if message.contains("Device or resource busy") {
                    DurgodError::DeviceBusy
                } else {
                    DurgodError::CommunicationError(message)
                }
            }
            hidapi::HidError::HidApiErrorEmpty => {
                DurgodError::CommunicationError("Unknown HID API error".to_string())
            }
            hidapi::HidError::FromWideCharError { wide_char } => {
                DurgodError::CommunicationError(format!("String conversion error: {}", wide_char))
            }
        }
    }
}

/// Result type for Durgod operations
pub type DurgodResult<T> = Result<T, DurgodError>; 