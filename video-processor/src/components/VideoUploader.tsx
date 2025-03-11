import React, { useState, useCallback } from 'react';

interface VideoUploaderProps {
  onVideoSelect: () => void;
}

const VideoUploader: React.FC<VideoUploaderProps> = ({ onVideoSelect }) => {
  const [isDragging, setIsDragging] = useState(false);

  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
    
    // We're not actually handling the drop here since we're using Tauri's dialog
    // But we could extend this in the future
    onVideoSelect();
  }, [onVideoSelect]);

  return (
    <div className="video-uploader">
      <div 
        className={`drop-zone ${isDragging ? 'dragging' : ''}`}
        onDragEnter={handleDragEnter}
        onDragLeave={handleDragLeave}
        onDragOver={handleDragOver}
        onDrop={handleDrop}
        onClick={onVideoSelect}
      >
        <div className="upload-icon">
          <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="17 8 12 3 7 8"></polyline>
            <line x1="12" y1="3" x2="12" y2="15"></line>
          </svg>
        </div>
        <h2>Select a Video File</h2>
        <p>Click or drag and drop to select a video file (MP4, AVI, or MOV)</p>
        <button className="select-button">Select Video</button>
      </div>
    </div>
  );
};

export default VideoUploader; 