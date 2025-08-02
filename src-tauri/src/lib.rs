use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use std::io::Read;
use uuid::Uuid;
use log::{info, warn, error, debug};

mod config;
mod constants;
mod errors;
mod validation;

use config::AppConfig;
use constants::*;
use errors::{AppError, AppResult};
use validation::ValidationHelper;

// Validate and sanitize file paths using validation helper
fn validate_file_path(file_path: &str) -> AppResult<PathBuf> {
    ValidationHelper::validate_path_safety(file_path)
}

// Validate HEIC/HEIF file by checking magic bytes
fn validate_heic_file(file_path: &Path) -> AppResult<()> {
    let mut file = fs::File::open(file_path)?;
    
    let mut buffer = [0u8; 12];
    if file.read_exact(&mut buffer).is_err() {
        return Err(AppError::InvalidHeicFile("File too small or unreadable".to_string()));
    }
    
    // Check for HEIC/HEIF magic bytes
    if buffer[HEIC_MAGIC_OFFSET..HEIC_MAGIC_OFFSET + HEIC_MAGIC_SIZE] == *HEIC_MAGIC_BYTES {
        let brand = &buffer[8..12];
        for supported_brand in HEIC_BRANDS {
            if brand.starts_with(supported_brand) {
                return Ok(());
            }
        }
    }
    
    Err(AppError::InvalidHeicFile("Invalid HEIC/HEIF magic bytes".to_string()))
}

#[tauri::command]
async fn save_temp_file(file_name: String, file_data: Vec<u8>) -> Result<String, String> {
    // Validate file name
    if let Err(e) = ValidationHelper::validate_file_name(&file_name) {
        error!("Invalid file name: {}", e);
        return Err(e.into());
    }
    
    let temp_filename = format!("{}_{}", Uuid::new_v4(), file_name);
    let temp_path = std::env::temp_dir().join(&temp_filename);
    
    match fs::write(&temp_path, file_data) {
        Ok(_) => Ok(temp_path.to_string_lossy().to_string()),
        Err(_) => Err(ERROR_TEMP_FILE_FAILED.to_string()),
    }
}

#[tauri::command]
async fn get_file_size(file_path: String) -> Result<u64, String> {
    let path = Path::new(&file_path);
    match fs::metadata(path) {
        Ok(metadata) => Ok(metadata.len()),
        Err(_) => Err(ERROR_FILE_NOT_FOUND.to_string()),
    }
}

#[tauri::command]
async fn convert_heic_to_jpg(file_path: String) -> Result<String, String> {
    match convert_heic_to_jpg_internal(file_path).await {
        Ok(path) => Ok(path),
        Err(e) => Err(e.into()),
    }
}

async fn convert_heic_to_jpg_internal(file_path: String) -> AppResult<String> {
    info!("Starting HEIC to JPEG conversion for: {}", file_path);
    let config = AppConfig::load();
    debug!("Using config: max_file_size={}MB, jpeg_quality={}", 
           config.conversion.max_file_size_mb, config.conversion.jpeg_quality);
    
    // Validate and sanitize the input path
    let input_path = validate_file_path(&file_path)?;
    
    // Check file size limit (configurable)
    let max_file_size = config.max_file_size_bytes();
    let metadata = fs::metadata(&input_path)?;
    let file_size_mb = metadata.len() / (1024 * 1024);
    debug!("File size: {}MB", file_size_mb);
    
    ValidationHelper::validate_file_size(metadata.len(), max_file_size)
        .map_err(|e| {
            warn!("File size validation failed: {}", e);
            e
        })?;
    
    // Validate file extension using validation helper
    ValidationHelper::validate_extension(&input_path)?;
    
    // Validate file content by checking magic bytes
    validate_heic_file(&input_path)?;
    
    // Create a unique temporary filename for the converted file
    let temp_filename = format!("{}_converted.jpg", Uuid::new_v4());
    
    // Always save to temp directory for temporary storage
    let temp_dir = std::env::temp_dir();
    let output_path = temp_dir.join(&temp_filename);
    
    convert_heic_file(&input_path, &output_path, &config)?;
    let output_path_str = output_path.to_string_lossy().to_string();
    info!("Conversion completed successfully: {}", output_path_str);
    Ok(output_path_str)
}

fn convert_heic_file(input_path: &Path, output_path: &Path, config: &AppConfig) -> AppResult<()> {
    #[cfg(target_os = "macos")]
    {
        debug!("Executing sips command with quality {}", config.conversion.jpeg_quality);
        let output = Command::new("sips")
            .arg("-s")
            .arg("format")
            .arg("jpeg")
            .arg("-s")
            .arg("formatOptions")
            .arg(config.jpeg_quality_string())
            .arg(input_path)
            .arg("--out")
            .arg(output_path)
            .output()
            .map_err(|e| AppError::ConversionFailed(format!("Failed to execute sips: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("sips command failed: {}", stderr);
            return Err(AppError::ConversionFailed(
                format!("sips command failed: {}", stderr)
            ));
        }
        
        Ok(())
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        if let Ok(img) = image::open(input_path) {
            img.save(output_path)?;
            Ok(())
        } else {
            Err(AppError::ConversionFailed(
                "HEIC format not supported on this platform. Please use macOS with sips.".to_string()
            ))
        }
    }
}

#[tauri::command]
async fn download_file(temp_file_path: String, save_path: String) -> Result<(), String> {
    let temp_path = Path::new(&temp_file_path);
    let save_path = Path::new(&save_path);
    
    if !temp_path.exists() {
        return Err(ERROR_FILE_NOT_FOUND.to_string());
    }
    
    // Check if destination file already exists (though dialog should handle this)
    if save_path.exists() {
        // The file dialog should have already confirmed overwrite, so we proceed
        // but we could add additional logging here if needed
    }
    
    // Ensure parent directory exists
    if let Some(parent) = save_path.parent() {
        if !parent.exists() {
            if let Err(_) = fs::create_dir_all(parent) {
                return Err(ERROR_SAVE_FAILED.to_string());
            }
        }
    }
    
    // Copy the temp file to the user's chosen location
    match fs::copy(temp_path, save_path) {
        Ok(_) => Ok(()),
        Err(_) => Err(ERROR_SAVE_FAILED.to_string()),
    }
}

#[tauri::command]
async fn cleanup_temp_file(file_path: String) -> Result<(), String> {
    let path = Path::new(&file_path);
    
    if path.exists() {
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to cleanup temporary file".to_string()),
        }
    } else {
        Ok(()) // File doesn't exist, consider it cleaned up
    }
}

#[tauri::command]
async fn get_app_config() -> Result<AppConfig, String> {
    Ok(AppConfig::load())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
        
    info!("Starting HEIC Converter application");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![save_temp_file, convert_heic_to_jpg, download_file, cleanup_temp_file, get_file_size, get_app_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
