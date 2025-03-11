import React from 'react';
import { CutPoint } from '../types';

interface ProcessingOptionsProps {
  cutPoints: CutPoint[];
  updateCutPoint: (index: number, cutPoint: CutPoint) => void;
  addCutPoint: () => void;
  removeCutPoint: (index: number) => void;
  applyZoomEffects: boolean;
  setApplyZoomEffects: (value: boolean) => void;
  onProcess: () => void;
}

const ProcessingOptions: React.FC<ProcessingOptionsProps> = ({
  cutPoints,
  updateCutPoint,
  addCutPoint,
  removeCutPoint,
  applyZoomEffects,
  setApplyZoomEffects,
  onProcess
}) => {
  // Format time in seconds to MM:SS format for display
  const formatTime = (timeInSeconds: number): string => {
    const minutes = Math.floor(timeInSeconds / 60);
    const seconds = Math.floor(timeInSeconds % 60);
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  };

  // Parse time from MM:SS format to seconds
  const parseTime = (timeString: string): number => {
    const [minutes, seconds] = timeString.split(':').map(Number);
    return minutes * 60 + seconds;
  };

  // Handle time input change
  const handleTimeChange = (index: number, field: 'startTime' | 'endTime', value: string) => {
    try {
      const timeInSeconds = parseTime(value);
      const updatedCutPoint = { ...cutPoints[index], [field]: timeInSeconds };
      updateCutPoint(index, updatedCutPoint);
    } catch (error) {
      // Invalid time format, ignore
    }
  };

  // Handle description change
  const handleDescriptionChange = (index: number, value: string) => {
    const updatedCutPoint = { ...cutPoints[index], description: value };
    updateCutPoint(index, updatedCutPoint);
  };

  return (
    <div className="processing-options">
      <h2>Processing Options</h2>
      
      <div className="cut-points-container">
        <h3>Cut Points</h3>
        
        {cutPoints.length === 0 ? (
          <p className="no-cut-points">No cut points defined. Add a cut point to start.</p>
        ) : (
          <div className="cut-points-list">
            {cutPoints.map((cutPoint, index) => (
              <div key={index} className="cut-point-item">
                <div className="cut-point-header">
                  <h4>Segment {index + 1}</h4>
                  <button 
                    className="remove-button"
                    onClick={() => removeCutPoint(index)}
                  >
                    Remove
                  </button>
                </div>
                
                <div className="cut-point-inputs">
                  <div className="input-group">
                    <label>Start Time (MM:SS)</label>
                    <input
                      type="text"
                      value={formatTime(cutPoint.startTime)}
                      onChange={(e) => handleTimeChange(index, 'startTime', e.target.value)}
                      placeholder="00:00"
                    />
                  </div>
                  
                  <div className="input-group">
                    <label>End Time (MM:SS)</label>
                    <input
                      type="text"
                      value={formatTime(cutPoint.endTime)}
                      onChange={(e) => handleTimeChange(index, 'endTime', e.target.value)}
                      placeholder="00:00"
                    />
                  </div>
                  
                  <div className="input-group full-width">
                    <label>Description</label>
                    <input
                      type="text"
                      value={cutPoint.description}
                      onChange={(e) => handleDescriptionChange(index, e.target.value)}
                      placeholder="Segment description"
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
        
        <button 
          className="add-button"
          onClick={addCutPoint}
        >
          Add Cut Point
        </button>
      </div>
      
      <div className="effects-options">
        <h3>Effects</h3>
        
        <div className="checkbox-group">
          <input
            type="checkbox"
            id="zoom-effects"
            checked={applyZoomEffects}
            onChange={(e) => setApplyZoomEffects(e.target.checked)}
          />
          <label htmlFor="zoom-effects">Apply zoom effects on faces during cuts</label>
        </div>
      </div>
      
      <div className="process-actions">
        <button 
          className="process-button"
          onClick={onProcess}
          disabled={cutPoints.length === 0}
        >
          Process Video
        </button>
      </div>
    </div>
  );
};

export default ProcessingOptions; 