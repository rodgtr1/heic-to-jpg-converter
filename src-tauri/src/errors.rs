use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum AppError {
    FileNotFound(String),
    InvalidPath(String),
    FileTooLarge { 
        file_size_mb: u64, 
        max_size_mb: u64 
    },
    InvalidHeicFile(String),
    ConversionFailed(String),
    SaveFailed(String),
    TempFileFailed(String),
    ConfigError(String),
    IoError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileNotFound(path) => write!(f, "File not found: {}", path),
            AppError::InvalidPath(path) => write!(f, "Invalid file path: {}", path),
            AppError::FileTooLarge { file_size_mb, max_size_mb } => {
                write!(f, "File size {}MB exceeds maximum {}MB", file_size_mb, max_size_mb)
            }
            AppError::InvalidHeicFile(reason) => write!(f, "Invalid HEIC/HEIF file: {}", reason),
            AppError::ConversionFailed(reason) => write!(f, "Conversion failed: {}", reason),
            AppError::SaveFailed(reason) => write!(f, "Save failed: {}", reason),
            AppError::TempFileFailed(reason) => write!(f, "Temporary file operation failed: {}", reason),
            AppError::ConfigError(reason) => write!(f, "Configuration error: {}", reason),
            AppError::IoError(reason) => write!(f, "I/O error: {}", reason),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::ConfigError(error.to_string())
    }
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

pub type AppResult<T> = Result<T, AppError>;