export interface CutPoint {
  startTime: number;
  endTime: number;
  description: string;
}

export interface VideoProcessingOptions {
  outputPath: string;
  cutPoints: CutPoint[];
  applyZoomEffects: boolean;
}

export interface TranscriptSegment {
  start: number;
  end: number;
  text: string;
}

export interface Transcript {
  segments: TranscriptSegment[];
  text: string;
} 