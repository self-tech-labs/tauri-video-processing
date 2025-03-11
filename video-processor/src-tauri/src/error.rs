use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("FFmpeg error: {0}")]
    FFmpegError(String),
    
    #[error("Whisper error: {0}")]
    WhisperError(String),
    
    #[error("Video processing error: {0}")]
    VideoProcessingError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::Other(error.to_string())
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Other(error)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        AppError::Other(error.to_string())
    }
} 