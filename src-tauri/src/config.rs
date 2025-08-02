use serde::{Deserialize, Serialize};
use std::fs;
use crate::constants::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    #[serde(rename = "jpegQuality")]
    pub jpeg_quality: u8,
    #[serde(rename = "maxFileSizeMB")]
    pub max_file_size_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(rename = "windowWidth")]
    pub window_width: u32,
    #[serde(rename = "windowHeight")]
    pub window_height: u32,
    #[serde(rename = "maxConcurrentConversions")]
    pub max_concurrent_conversions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    #[serde(rename = "cleanupTempFiles")]
    pub cleanup_temp_files: bool,
    #[serde(rename = "tempFileRetentionHours")]
    pub temp_file_retention_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub conversion: ConversionConfig,
    pub ui: UiConfig,
    pub storage: StorageConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            conversion: ConversionConfig {
                jpeg_quality: DEFAULT_JPEG_QUALITY,
                max_file_size_mb: DEFAULT_MAX_FILE_SIZE_MB,
            },
            ui: UiConfig {
                window_width: DEFAULT_WINDOW_WIDTH,
                window_height: DEFAULT_WINDOW_HEIGHT,
                max_concurrent_conversions: DEFAULT_MAX_CONCURRENT_CONVERSIONS,
            },
            storage: StorageConfig {
                cleanup_temp_files: DEFAULT_CLEANUP_TEMP_FILES,
                temp_file_retention_hours: DEFAULT_TEMP_FILE_RETENTION_HOURS,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        // Try to load from config.json, fall back to defaults
        if let Ok(config_str) = fs::read_to_string("config.json") {
            if let Ok(config) = serde_json::from_str(&config_str) {
                return config;
            }
        }
        
        // Try environment variables for key settings
        let mut config = Self::default();
        
        if let Ok(quality_str) = std::env::var("HEIC_JPEG_QUALITY") {
            if let Ok(quality) = quality_str.parse::<u8>() {
                if quality > 0 && quality <= 100 {
                    config.conversion.jpeg_quality = quality;
                }
            }
        }
        
        if let Ok(size_str) = std::env::var("HEIC_MAX_FILE_SIZE_MB") {
            if let Ok(size) = size_str.parse::<u64>() {
                if size > 0 {
                    config.conversion.max_file_size_mb = size;
                }
            }
        }
        
        config
    }
    
    pub fn max_file_size_bytes(&self) -> u64 {
        self.conversion.max_file_size_mb * 1024 * 1024
    }
    
    pub fn jpeg_quality_string(&self) -> String {
        self.conversion.jpeg_quality.to_string()
    }
}