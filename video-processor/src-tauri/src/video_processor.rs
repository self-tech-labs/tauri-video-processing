use crate::error::AppError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoProcessingOptions {
    pub output_path: String,
    pub cut_points: Vec<CutPoint>,
    pub apply_zoom_effects: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CutPoint {
    pub start_time: f64,
    pub end_time: f64,
    pub description: String,
}

/// Extracts audio from a video file using FFmpeg
pub fn extract_audio(video_path: &str) -> Result<String, AppError> {
    let video_path = Path::new(video_path);
    let file_stem = video_path.file_stem()
        .ok_or_else(|| AppError::VideoProcessingError("Invalid video file path".to_string()))?;
    
    let output_dir = video_path.parent()
        .ok_or_else(|| AppError::VideoProcessingError("Invalid video file path".to_string()))?;
    
    let audio_path = output_dir.join(format!("{}_audio.wav", file_stem.to_string_lossy()));
    
    // Use FFmpeg to extract audio
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg("-vn") // Disable video
        .arg("-acodec")
        .arg("pcm_s16le") // Use PCM 16-bit format for Whisper
        .arg("-ar")
        .arg("16000") // 16kHz sample rate for Whisper
        .arg("-ac")
        .arg("1") // Mono audio
        .arg("-y") // Overwrite output file if it exists
        .arg(&audio_path)
        .status()
        .map_err(|e| AppError::FFmpegError(format!("Failed to run FFmpeg: {}", e)))?;
    
    if !status.success() {
        return Err(AppError::FFmpegError("FFmpeg command failed".to_string()));
    }
    
    Ok(audio_path.to_string_lossy().to_string())
}

/// Process video based on transcript and cut points
pub fn process_video(
    video_path: &str,
    _transcript_path: &str,
    options: VideoProcessingOptions,
) -> Result<String, AppError> {
    // Create a Python script to process the video with MoviePy
    let python_script = create_moviepy_script(video_path, &options)?;
    let script_file = NamedTempFile::new()
        .map_err(|e| AppError::IoError(e))?;
    let script_path = script_file.path().to_string_lossy().to_string();
    
    // Write the Python script to a temporary file
    fs::write(&script_path, python_script)
        .map_err(|e| AppError::IoError(e))?;
    
    // Run the Python script
    let status = Command::new("python")
        .arg(&script_path)
        .status()
        .map_err(|e| AppError::VideoProcessingError(format!("Failed to run Python script: {}", e)))?;
    
    if !status.success() {
        return Err(AppError::VideoProcessingError("Python script failed".to_string()));
    }
    
    Ok(options.output_path)
}

/// Create a Python script for MoviePy to process the video
fn create_moviepy_script(video_path: &str, options: &VideoProcessingOptions) -> Result<String, AppError> {
    let mut script = String::from(r#"
import sys
from moviepy.editor import VideoFileClip, concatenate_videoclips

def process_video():
    # Load the video file
    video = VideoFileClip("VIDEO_PATH")
    
    # Define cut points
    cut_points = CUT_POINTS
    
    # Create subclips
    clips = []
    for cut in cut_points:
        start_time = cut["start_time"]
        end_time = cut["end_time"]
        clip = video.subclip(start_time, end_time)
        
        # Apply zoom effects if enabled
        if APPLY_ZOOM_EFFECTS:
            # This is a simple zoom effect, can be customized
            clip = clip.fx(lambda c: c.resize(1.1))
        
        clips.append(clip)
    
    # Concatenate clips
    final_clip = concatenate_videoclips(clips)
    
    # Write the result
    final_clip.write_videofile("OUTPUT_PATH", codec="libx264")
    
    # Close clips
    video.close()
    final_clip.close()

if __name__ == "__main__":
    process_video()
"#);
    
    // Replace placeholders with actual values
    script = script.replace("VIDEO_PATH", video_path);
    script = script.replace("OUTPUT_PATH", &options.output_path);
    script = script.replace("CUT_POINTS", &serde_json::to_string(&options.cut_points)
        .map_err(|e| AppError::VideoProcessingError(format!("Failed to serialize cut points: {}", e)))?);
    script = script.replace("APPLY_ZOOM_EFFECTS", &options.apply_zoom_effects.to_string());
    
    Ok(script)
}

/// Check if FFmpeg is installed
pub async fn check_ffmpeg_installed() -> bool {
    let command = Command::new("ffmpeg")
        .arg("-version")
        .output();
    
    match command {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Check if Python and MoviePy are installed
pub async fn check_moviepy_installed() -> bool {
    let command = Command::new("python")
        .arg("-c")
        .arg("import moviepy")
        .output();
    
    match command {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
} 