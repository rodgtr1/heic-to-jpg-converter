# Contributing to HEIC Converter

Thank you for your interest in contributing to the HEIC Converter project! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Process](#contributing-process)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

This project follows a simple code of conduct:
- Be respectful and inclusive
- Focus on constructive feedback
- Help maintain a welcoming environment for all contributors

## Getting Started

### Prerequisites

- **Node.js** (v18 or later)
- **Rust** (latest stable version)
- **macOS** (for full functionality - the app uses macOS `sips` command)

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/heic-converter.git
   cd heic-converter
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri dev
   ```

4. **Build for production**
   ```bash
   npm run tauri build
   ```

## Contributing Process

1. **Check existing issues** - Look for existing issues or feature requests
2. **Create an issue** - If none exists, create one describing the bug/feature
3. **Fork the repository** - Create your own fork to work on
4. **Create a branch** - Use a descriptive branch name
5. **Make changes** - Implement your changes following our guidelines
6. **Test thoroughly** - Ensure your changes work correctly
7. **Submit a pull request** - Describe your changes clearly

## Code Style

### Rust Code

- Follow standard Rust formatting with `rustfmt`
- Use meaningful variable and function names
- Add documentation comments for public functions
- Handle errors appropriately using our `AppError` types
- Use structured logging with appropriate log levels

Example:
```rust
/// Validates HEIC file format by checking magic bytes
fn validate_heic_file(file_path: &Path) -> AppResult<()> {
    debug!("Validating HEIC file: {}", file_path.display());
    // Implementation...
}
```

### TypeScript/React Code

- Use TypeScript strict mode
- Follow React hooks best practices
- Use meaningful component and variable names
- Handle errors gracefully with user-friendly messages

Example:
```typescript
const processFile = useCallback(async (id: string) => {
  updateFileItem(id, { status: 'processing', progress: 10 });
  
  try {
    // Processing logic...
  } catch (error) {
    updateFileItem(id, { 
      status: 'failed', 
      errorMessage: `Conversion failed: ${String(error)}` 
    });
  }
}, []);
```

### Configuration

- Use the configuration system for adjustable values
- Don't hard-code limits or quality settings
- Document configuration options

## Testing

### Manual Testing

1. **File Conversion Testing**
   - Test with various HEIC file sizes
   - Test with different file formats (should fail gracefully)
   - Test file size limits
   - Test batch processing

2. **Error Handling Testing**
   - Test with invalid files
   - Test with files exceeding size limits
   - Test with insufficient permissions
   - Test with corrupted HEIC files

3. **UI Testing**
   - Test file dialog functionality
   - Test progress indicators
   - Test download functionality
   - Test error message display

### Configuration Testing

Test different configuration values:
```json
{
  "conversion": {
    "jpegQuality": 75,
    "maxFileSizeMB": 50
  }
}
```

## Submitting Changes

### Pull Request Guidelines

1. **Clear title and description**
   - Summarize the change in the title
   - Provide detailed description of what and why

2. **Reference issues**
   - Link to related issues: "Fixes #123" or "Addresses #456"

3. **Test your changes**
   - Ensure the app builds successfully
   - Test the affected functionality manually
   - Verify no regressions in existing features

4. **Keep changes focused**
   - One feature/fix per pull request
   - Avoid mixing unrelated changes

### Example Pull Request Template

```markdown
## Summary
Brief description of the changes made.

## Changes Made
- [ ] Added new feature X
- [ ] Fixed bug Y
- [ ] Updated documentation Z

## Testing
- [ ] Tested with various HEIC files
- [ ] Verified error handling
- [ ] Confirmed no regressions

## Related Issues
Fixes #123
```

## Project Structure

```
heic-converter/
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   ├── App.css            # Styling
│   └── main.tsx           # Application entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Main Tauri commands
│   │   ├── config.rs      # Configuration management
│   │   ├── constants.rs   # Application constants
│   │   ├── errors.rs      # Error types
│   │   └── validation.rs  # Input validation helpers
│   ├── config.json        # Default configuration
│   └── Cargo.toml         # Rust dependencies
├── README.md              # Project documentation
├── CONTRIBUTING.md        # This file
└── LICENSE                # MIT license
```

## Configuration System

The app uses a layered configuration system:

1. **Default values** - Defined in `constants.rs`
2. **Config file** - `src-tauri/config.json`
3. **Environment variables** - `HEIC_JPEG_QUALITY`, `HEIC_MAX_FILE_SIZE_MB`

When adding new configuration options:
- Add constants to `constants.rs`
- Update the config structures in `config.rs`
- Document the option in README.md

## Error Handling

Use structured error types from `errors.rs`:

```rust
// Good
return Err(AppError::FileTooLarge {
    file_size_mb: actual_size,
    max_size_mb: limit,
});

// Avoid
return Err("File too large".to_string());
```

## Logging

Use appropriate log levels:
- `error!()` - For errors that prevent operation
- `warn!()` - For potentially problematic situations
- `info!()` - For general information about operations
- `debug!()` - For detailed debugging information

## Questions?

If you have questions about contributing:
1. Check existing issues and discussions
2. Create an issue with the "question" label
3. Be specific about what you're trying to achieve

Thank you for contributing to HEIC Converter!