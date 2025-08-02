# HEIC Converter

A fast and simple desktop application for converting HEIC/HEIF images to high-quality JPEG format. Built with Tauri and React for optimal performance and native feel.

## Features

- **Batch Processing**: Convert multiple HEIC files simultaneously
- **Progress Tracking**: Real-time progress bars for each file conversion
- **High Quality Output**: 90% JPEG quality for excellent image quality
- **File Size Limit**: 100MB maximum file size for security
- **Native macOS Integration**: Uses built-in `sips` command for fast conversion
- **Intuitive Interface**: Simple file dialog selection and download workflow
- **Secure**: Path validation, file validation, and temporary file cleanup

## System Requirements

- **macOS**: Requires macOS (uses native `sips` command for HEIC conversion)
- **Storage**: Minimal disk space required (temporary files are cleaned up automatically)

## Installation

### Option 1: Download Pre-built App
1. Download the latest release from the releases page
2. Move the app to your Applications folder
3. Right-click and select "Open" to bypass Gatekeeper on first launch

### Option 2: Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/heic-converter.git
cd heic-converter

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## How to Use

1. **Launch the App**: Open HEIC Converter from your Applications folder
2. **Select Files**: Click "Select HEIC Files" to choose one or more HEIC/HEIF images
3. **Monitor Progress**: Watch the progress bars as files are converted automatically
4. **Download**: Click "Download" next to completed files to save them as JPEG
5. **Add More**: Use "+ Add More Files" to add additional files to the queue

## File Specifications

- **Supported Input**: HEIC, HEIF files
- **Output Format**: JPEG (90% quality)
- **Maximum File Size**: 100MB per file
- **Validation**: Both file extension and magic byte validation for security

## Technical Details

- **Framework**: Tauri v2 with React frontend
- **Backend**: Rust with native macOS `sips` integration
- **Security**: Path validation, file size limits, temporary file cleanup
- **Performance**: Optimized build settings for minimal bundle size

## Development

### Project Structure
```
heic-converter/
├── src/                 # React frontend
├── src-tauri/          # Rust backend
├── public/             # Static assets
└── README.md
```

### Available Scripts
- `npm run dev` - Start development server
- `npm run build` - Build frontend for production
- `npm run tauri dev` - Run Tauri development mode
- `npm run tauri build` - Build production app

### Recommended IDE Setup
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Troubleshooting

### Common Issues

**App won't open on macOS**
- Right-click the app and select "Open" to bypass Gatekeeper
- Check System Preferences > Security & Privacy if blocked

**File conversion fails**
- Ensure the file is actually HEIC/HEIF format
- Check that file size is under 100MB
- Verify you have write permissions to save location

**Performance issues**
- Close other applications to free up system resources
- Avoid converting many very large files simultaneously

### Error Messages

- `File exceeds 100MB size limit` - Choose smaller files
- `File is not a valid HEIC/HEIF image` - Check file format
- `Invalid file path` - Choose files from accessible locations
- `Failed to save converted file` - Check save location permissions

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/) framework
- Uses macOS native `sips` command for optimal performance
- React frontend for modern user interface
