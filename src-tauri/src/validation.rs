use crate::constants::*;
use crate::errors::{AppError, AppResult};
use std::path::{Path, PathBuf};

pub struct ValidationHelper;

impl ValidationHelper {
    /// Validate file extension against supported formats
    pub fn validate_extension(file_path: &Path) -> AppResult<()> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        if !SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(AppError::InvalidHeicFile(
                format!("Unsupported extension '{}'. Supported: {}", 
                       extension, 
                       SUPPORTED_EXTENSIONS.join(", "))
            ));
        }
        
        Ok(())
    }
    
    /// Validate file size against configured limits
    pub fn validate_file_size(file_size: u64, max_size: u64) -> AppResult<()> {
        if file_size > max_size {
            return Err(AppError::FileTooLarge {
                file_size_mb: file_size / (1024 * 1024),
                max_size_mb: max_size / (1024 * 1024),
            });
        }
        Ok(())
    }
    
    /// Validate JPEG quality parameter (currently unused but kept for future use)
    #[allow(dead_code)]
    pub fn validate_jpeg_quality(quality: u8) -> AppResult<()> {
        if quality == 0 || quality > 100 {
            return Err(AppError::ConfigError(
                format!("JPEG quality must be between 1-100, got: {}", quality)
            ));
        }
        Ok(())
    }
    
    /// Validate file name for temporary file creation
    pub fn validate_file_name(file_name: &str) -> AppResult<()> {
        if file_name.is_empty() {
            return Err(AppError::InvalidPath("File name cannot be empty".to_string()));
        }
        
        // Check for dangerous characters
        let dangerous_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if file_name.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err(AppError::InvalidPath(
                "File name contains invalid characters".to_string()
            ));
        }
        
        // Check for overly long names
        if file_name.len() > 255 {
            return Err(AppError::InvalidPath(
                "File name too long (max 255 characters)".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Validate path for safety (no traversal, exists, is file)
    pub fn validate_path_safety(path: &str) -> AppResult<PathBuf> {
        if path.contains("..") {
            return Err(AppError::InvalidPath(
                "Path traversal not allowed".to_string()
            ));
        }
        
        let path_buf = Path::new(path);
        let canonical_path = path_buf.canonicalize()
            .map_err(|_| AppError::InvalidPath("Cannot resolve path".to_string()))?;
        
        if !canonical_path.is_file() {
            return Err(AppError::FileNotFound(path.to_string()));
        }
        
        Ok(canonical_path)
    }
}