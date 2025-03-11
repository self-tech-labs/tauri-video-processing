import React, { useEffect, useRef } from 'react';
import { convertFileSrc } from '@tauri-apps/api/core';

interface VideoPreviewProps {
  videoPath: string;
}

const VideoPreview: React.FC<VideoPreviewProps> = ({ videoPath }) => {
  const videoRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    // Convert the file path to a URL that can be used in the browser
    const videoUrl = convertFileSrc(videoPath);
    
    if (videoRef.current) {
      videoRef.current.src = videoUrl;
    }
  }, [videoPath]);

  return (
    <div className="video-preview">
      <h2>Processed Video</h2>
      
      <div className="video-container">
        <video 
          ref={videoRef}
          controls
          className="video-player"
        >
          Your browser does not support the video tag.
        </video>
      </div>
      
      <div className="video-actions">
        <p>Your video has been processed successfully!</p>
        <p className="video-path">Saved to: {videoPath}</p>
      </div>
    </div>
  );
};

export default VideoPreview; 