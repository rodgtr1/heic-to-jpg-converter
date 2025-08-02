import { useState, useCallback, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// Configuration constants - these should ideally come from a config API
const CONFIG = {
  MAX_FILE_SIZE_MB: 100, // This will be overridden by backend validation
  MAX_CONCURRENT_CONVERSIONS: 5,
};


interface FileQueueItem {
  id: string;
  name: string;
  size: number;
  status: 'queued' | 'processing' | 'completed' | 'failed';
  progress: number;
  downloadPath?: string;
  errorMessage?: string;
}

interface AppState {
  fileQueue: FileQueueItem[];
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

function createMockFileFromPath(path: string, fileName: string, fileSize: number): File {
  const mockFile = new File([''], fileName, { type: 'image/heic' });
  (mockFile as any).path = path;
  Object.defineProperty(mockFile, 'size', { value: fileSize, writable: false });
  return mockFile;
}

function App() {
  const [state, setState] = useState<AppState>({
    fileQueue: []
  });

  // Store files separately to avoid serialization issues
  const fileStore = useRef<Map<string, File>>(new Map());

  // Cleanup temp files when component unmounts (app closes)
  useEffect(() => {
    const cleanupAllTempFiles = async () => {
      for (const item of state.fileQueue) {
        if (item.downloadPath) {
          await cleanupTempFile(item.downloadPath);
        }
      }
    };

    // Cleanup on app close
    const handleBeforeUnload = () => {
      cleanupAllTempFiles();
    };
    
    window.addEventListener('beforeunload', handleBeforeUnload);

    return () => {
      window.removeEventListener('beforeunload', handleBeforeUnload);
      cleanupAllTempFiles();
    };
  }, [state.fileQueue]);

  const addFilesToQueue = (files: File[]) => {
    const MAX_FILE_SIZE = CONFIG.MAX_FILE_SIZE_MB * 1024 * 1024; // Convert MB to bytes
    
    const newItems: FileQueueItem[] = files.map(file => {
      const id = crypto.randomUUID();
      
      // Store file in separate ref to avoid serialization issues
      fileStore.current.set(id, file);
      
      // Check file size limit
      if (file.size > MAX_FILE_SIZE) {
        return {
          id,
          name: file.name,
          size: file.size,
          status: 'failed' as const,
          progress: 0,
          errorMessage: `File size exceeds ${CONFIG.MAX_FILE_SIZE_MB}MB limit`
        };
      }
      
      return {
        id,
        name: file.name,
        size: file.size,
        status: 'queued' as const,
        progress: 0
      };
    });
    
    setState(prev => ({
      ...prev,
      fileQueue: [...prev.fileQueue, ...newItems]
    }));
    
    // Start processing each file that isn't already failed
    newItems.forEach(item => {
      if (item.status !== 'failed') {
        processFile(item.id);
      }
    });
  };

  const updateFileItem = (id: string, updates: Partial<FileQueueItem>) => {
    setState(prev => ({
      ...prev,
      fileQueue: prev.fileQueue.map(item => 
        item.id === id ? { ...item, ...updates } : item
      )
    }));
  };

  const removeFileItem = async (id: string) => {
    // Cleanup temp file if it exists
    const item = state.fileQueue.find(item => item.id === id);
    if (item?.downloadPath) {
      await cleanupTempFile(item.downloadPath);
    }
    
    // Remove from file store
    fileStore.current.delete(id);
    
    setState(prev => ({
      ...prev,
      fileQueue: prev.fileQueue.filter(item => item.id !== id)
    }));
  };

  const processFile = useCallback(async (id: string) => {
    updateFileItem(id, { status: 'processing', progress: 10 });

    try {
      // Get the file from the file store
      const file = fileStore.current.get(id);

      if (!file) {
        throw new Error('File not found in file store');
      }
      
      // Try to get the file path directly (this works on desktop)
      const filePath = (file as any).path;
      let pathToUse = filePath;
      
      updateFileItem(id, { progress: 30 });
      
      if (!filePath) {
        // Fallback: save the file data to temp location
        const arrayBuffer = await file.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const tempPath = await invoke("save_temp_file", { 
          fileName: file.name,
          fileData: Array.from(uint8Array)
        });
        pathToUse = tempPath;
      }
      
      updateFileItem(id, { progress: 60 });
      
      const outputPath = await invoke("convert_heic_to_jpg", { filePath: pathToUse }) as string;
      
      updateFileItem(id, { 
        status: 'completed', 
        progress: 100, 
        downloadPath: outputPath 
      });
    } catch (error) {
      // Cleanup any temp files created during failed conversion
      try {
        const file = fileStore.current.get(id);
        if (file && !((file as any).path)) {
          // If we created a temp file from file data, clean it up
          await invoke("cleanup_temp_file", { filePath: `temp file for ${file.name}` }).catch(() => {});
        }
      } catch (cleanupError) {
        // Silently handle cleanup errors
      }
      
      updateFileItem(id, { 
        status: 'failed', 
        progress: 0, 
        errorMessage: `Conversion failed: ${String(error)}` 
      });
    }
  }, []);



  const downloadFile = async (tempFilePath: string, originalFileName: string) => {
    try {
      // Use the save dialog to let user choose where to save
      const savePath = await invoke("plugin:dialog|save", {
        options: {
          defaultPath: originalFileName.replace(/\.heic$/i, '.jpg').replace(/\.heif$/i, '.jpg'),
          filters: [
            {
              name: "JPEG Images",
              extensions: ["jpg", "jpeg"]
            }
          ]
        }
      });
      
      if (savePath) {
        await invoke("download_file", { 
          tempFilePath: tempFilePath, 
          savePath: savePath 
        });
      }
    } catch (error) {
      console.error('Download failed:', error);
    }
  };

  const cleanupTempFile = async (tempFilePath: string) => {
    try {
      await invoke("cleanup_temp_file", { filePath: tempFilePath });
    } catch (error) {
      console.error('Cleanup failed:', error);
    }
  };


  const openFileDialog = useCallback(async () => {
    try {
      const filePaths = await invoke("plugin:dialog|open", {
        options: {
          multiple: true,
          filters: [
            {
              name: "HEIC Images",
              extensions: ["heic", "heif"]
            }
          ]
        }
      });
      
      if (filePaths && Array.isArray(filePaths) && filePaths.length > 0) {
        // Create File objects from paths for consistency
        const filePromises = (filePaths as string[]).map(async (path: string) => {
          const fileName = path.split('/').pop() || 'unknown.heic';
          // Get file size but don't fail if we can't get it
          const fileSize = await invoke("get_file_size", { filePath: path }).catch(() => 0);
          
          return createMockFileFromPath(path, fileName, fileSize as number);
        });
        
        const files = await Promise.all(filePromises);
        addFilesToQueue(files);
      }
    } catch (error) {
      console.error('File dialog error:', error);
    }
  }, []);

  const FileListItem = ({ item }: { item: FileQueueItem }) => (
    <div className="file-item">
      <div className="file-info">
        <div className="file-name">{item.name}</div>
        <div className="file-size">{formatFileSize(item.size)}</div>
      </div>
      
      <div className="file-progress">
        {item.status === 'queued' && (
          <div className="status-badge queued">Queued</div>
        )}
        
        {item.status === 'processing' && (
          <>
            <div className="progress-bar">
              <div 
                className="progress-fill" 
                style={{ width: `${item.progress}%` }}
              />
            </div>
            <div className="status-text">Processing...</div>
          </>
        )}
        
        {item.status === 'completed' && (
          <>
            <div className="status-badge completed">✓ Completed</div>
            <div className="file-actions">
              <button 
                onClick={() => item.downloadPath && downloadFile(item.downloadPath, item.name)}
                className="download-btn small"
              >
                Download
              </button>
            </div>
          </>
        )}
        
        {item.status === 'failed' && (
          <>
            <div className="status-badge failed">✗ Failed</div>
            <div className="error-message">{item.errorMessage}</div>
          </>
        )}
      </div>
      
      <button 
        onClick={() => removeFileItem(item.id)}
        className="remove-btn"
        title="Remove file"
      >
        ✕
      </button>
    </div>
  );

  return (
    <main className="container">
      <h1>HEIC Converter</h1>
      <p>Select HEIC files to convert to JPEG</p>

      <div className={`drop-zone ${state.fileQueue.length > 0 ? 'has-files' : ''}`}>
        {state.fileQueue.length === 0 ? (
          <div className="drop-message">
            <button 
              onClick={openFileDialog}
              className="download-btn"
              style={{ margin: '1rem 0' }}
            >
              Select HEIC Files
            </button>
          </div>
        ) : (
          <div className="file-list">
            <div className="file-list-header">
              <span>{state.fileQueue.length} file(s)</span>
              <button 
                onClick={openFileDialog}
                className="add-more-btn"
              >
                + Add More Files
              </button>
            </div>
            
            {state.fileQueue.map(item => (
              <FileListItem key={item.id} item={item} />
            ))}
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
