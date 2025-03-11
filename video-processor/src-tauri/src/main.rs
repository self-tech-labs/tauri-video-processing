// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod video_processor;
mod whisper;
mod error;

use video_processor::{extract_audio, process_video, VideoProcessingOptions};
use whisper::transcribe_audio;
use error::AppError;

#[tauri::command]
async fn extract_audio_from_video(video_path: String) -> Result<String, AppError> {
    extract_audio(&video_path).map_err(AppError::from)
}

#[tauri::command]
async fn transcribe_audio_file(audio_path: String) -> Result<String, AppError> {
    transcribe_audio(&audio_path).map_err(AppError::from)
}

#[tauri::command]
async fn process_video_file(
    video_path: String,
    transcript_path: String,
    options: VideoProcessingOptions,
) -> Result<String, AppError> {
    process_video(&video_path, &transcript_path, options).map_err(AppError::from)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            extract_audio_from_video,
            transcribe_audio_file,
            process_video_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
