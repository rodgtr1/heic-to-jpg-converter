// Application constants and default values

// Error messages (only used ones kept)
pub const ERROR_FILE_NOT_FOUND: &str = "File not found or inaccessible";
pub const ERROR_SAVE_FAILED: &str = "Failed to save converted file";
pub const ERROR_TEMP_FILE_FAILED: &str = "Failed to create temporary file";

// File validation
pub const SUPPORTED_EXTENSIONS: &[&str] = &["heic", "heif"];
pub const HEIC_MAGIC_OFFSET: usize = 4;
pub const HEIC_MAGIC_SIZE: usize = 4;
pub const HEIC_MAGIC_BYTES: &[u8] = b"ftyp";
pub const HEIC_BRANDS: &[&[u8]] = &[
    b"heic", b"heix", b"hevc", b"hevx", b"mif1"
];

// Conversion defaults
pub const DEFAULT_JPEG_QUALITY: u8 = 90;
pub const DEFAULT_MAX_FILE_SIZE_MB: u64 = 100;

// UI defaults  
pub const DEFAULT_WINDOW_WIDTH: u32 = 600;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 500;
pub const DEFAULT_MAX_CONCURRENT_CONVERSIONS: u32 = 5;

// Storage defaults
pub const DEFAULT_TEMP_FILE_RETENTION_HOURS: u32 = 24;
pub const DEFAULT_CLEANUP_TEMP_FILES: bool = true;