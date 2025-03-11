import React, { useState } from 'react';
import { Transcript } from '../types';

interface TranscriptViewerProps {
  transcript: Transcript;
}

const TranscriptViewer: React.FC<TranscriptViewerProps> = ({ transcript }) => {
  const [searchTerm, setSearchTerm] = useState('');

  // Format time in seconds to MM:SS format
  const formatTime = (timeInSeconds: number): string => {
    const minutes = Math.floor(timeInSeconds / 60);
    const seconds = Math.floor(timeInSeconds % 60);
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  };

  // Filter segments based on search term
  const filteredSegments = transcript.segments.filter(segment => 
    segment.text.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="transcript-viewer">
      <h2>Transcript</h2>
      
      <div className="search-container">
        <input
          type="text"
          placeholder="Search transcript..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="search-input"
        />
      </div>
      
      <div className="transcript-content">
        {filteredSegments.length === 0 ? (
          <p className="no-results">No matching segments found.</p>
        ) : (
          filteredSegments.map((segment, index) => (
            <div key={index} className="transcript-segment">
              <div className="segment-time">
                {formatTime(segment.start)} - {formatTime(segment.end)}
              </div>
              <div className="segment-text">
                {segment.text}
              </div>
            </div>
          ))
        )}
      </div>
      
      <div className="transcript-summary">
        <p>Total segments: {transcript.segments.length}</p>
        <p>Total duration: {formatTime(transcript.segments[transcript.segments.length - 1]?.end || 0)}</p>
      </div>
    </div>
  );
};

export default TranscriptViewer; 