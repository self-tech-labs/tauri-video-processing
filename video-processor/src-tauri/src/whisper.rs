use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcript {
    pub segments: Vec<TranscriptSegment>,
    pub text: String,
}

/// Transcribe audio file using Whisper
pub fn transcribe_audio(audio_path: &str) -> Result<String, AppError> {
    // Load Whisper model
    let model_path = download_whisper_model()?;
    let ctx = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())
        .map_err(|e| AppError::WhisperError(format!("Failed to load Whisper model: {}", e)))?;
    
    // Set up parameters
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(true);
    params.set_language(Some("auto"));
    params.set_translate(false);
    
    // Run inference
    let audio_path = Path::new(audio_path);
    
    // Load audio data from file
    let audio_data = load_audio_file(audio_path)?;
    
    let mut state = ctx.create_state()
        .map_err(|e| AppError::WhisperError(format!("Failed to create Whisper state: {}", e)))?;
    
    state.full(params, &audio_data)
        .map_err(|e| AppError::WhisperError(format!("Failed to run Whisper inference: {}", e)))?;
    
    // Extract segments
    let num_segments = state.full_n_segments()
        .map_err(|e| AppError::WhisperError(format!("Failed to get number of segments: {}", e)))?;
    
    let mut transcript = Transcript {
        segments: Vec::new(),
        text: String::new(),
    };
    
    for i in 0..num_segments {
        let segment_text = state.full_get_segment_text(i)
            .map_err(|e| AppError::WhisperError(format!("Failed to get segment text: {}", e)))?;
        
        let start_timestamp = state.full_get_segment_t0(i)
            .map_err(|e| AppError::WhisperError(format!("Failed to get segment start time: {}", e)))?;
        
        let end_timestamp = state.full_get_segment_t1(i)
            .map_err(|e| AppError::WhisperError(format!("Failed to get segment end time: {}", e)))?;
        
        let segment = TranscriptSegment {
            start: start_timestamp as f64,
            end: end_timestamp as f64,
            text: segment_text.clone(),
        };
        
        transcript.segments.push(segment);
        transcript.text.push_str(&segment_text);
        transcript.text.push(' ');
    }
    
    // Save transcript to file
    let output_path = audio_path.with_extension("json");
    let transcript_json = serde_json::to_string_pretty(&transcript)
        .map_err(|e| AppError::WhisperError(format!("Failed to serialize transcript: {}", e)))?;
    
    let mut file = File::create(&output_path)
        .map_err(|e| AppError::IoError(e))?;
    
    file.write_all(transcript_json.as_bytes())
        .map_err(|e| AppError::IoError(e))?;
    
    Ok(output_path.to_string_lossy().to_string())
}

/// Download Whisper model if not already present
fn download_whisper_model() -> Result<String, AppError> {
    let model_dir = dirs::cache_dir()
        .ok_or_else(|| AppError::WhisperError("Failed to get cache directory".to_string()))?
        .join("whisper-models");
    
    std::fs::create_dir_all(&model_dir)
        .map_err(|e| AppError::IoError(e))?;
    
    let model_path = model_dir.join("ggml-base.en.bin");
    
    if !model_path.exists() {
        // Download the model
        let url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin";
        
        let status = Command::new("curl")
            .arg("-L")
            .arg(url)
            .arg("-o")
            .arg(&model_path)
            .status()
            .map_err(|e| AppError::WhisperError(format!("Failed to download Whisper model: {}", e)))?;
        
        if !status.success() {
            return Err(AppError::WhisperError("Failed to download Whisper model".to_string()));
        }
    }
    
    Ok(model_path.to_string_lossy().to_string())
}

/// Analyze transcript to determine strategic cut points
pub fn analyze_transcript_for_cuts(transcript_path: &str) -> Result<Vec<crate::video_processor::CutPoint>, AppError> {
    let transcript_content = std::fs::read_to_string(transcript_path)
        .map_err(|e| AppError::IoError(e))?;
    
    let transcript: Transcript = serde_json::from_str(&transcript_content)
        .map_err(|e| AppError::WhisperError(format!("Failed to parse transcript: {}", e)))?;
    
    // Simple algorithm to find natural breaks (pauses)
    let mut cut_points = Vec::new();
    let mut current_start = 0.0;
    
    for i in 1..transcript.segments.len() {
        let prev_segment = &transcript.segments[i - 1];
        let curr_segment = &transcript.segments[i];
        
        // If there's a pause of more than 1 second, consider it a cut point
        let pause_duration = curr_segment.start - prev_segment.end;
        if pause_duration > 1.0 {
            cut_points.push(crate::video_processor::CutPoint {
                start_time: current_start,
                end_time: prev_segment.end,
                description: format!("Segment {}", cut_points.len() + 1),
            });
            
            current_start = curr_segment.start;
        }
    }
    
    // Add the final segment
    if let Some(last_segment) = transcript.segments.last() {
        cut_points.push(crate::video_processor::CutPoint {
            start_time: current_start,
            end_time: last_segment.end,
            description: format!("Segment {}", cut_points.len() + 1),
        });
    }
    
    Ok(cut_points)
}

fn load_audio_file(path: &Path) -> Result<Vec<f32>, AppError> {
    // Open the media source
    let file = File::open(path)
        .map_err(|e| AppError::WhisperError(format!("Failed to open audio file: {}", e)))?;
    
    // Create a media source stream
    let mss = MediaSourceStream::new(
        Box::new(file),
        Default::default(),
    );
    
    // Create a hint to help the format registry guess what format the media is
    let mut hint = Hint::new();
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            hint.with_extension(ext_str);
        }
    }
    
    // Use the default options for format reader
    let format_opts = FormatOptions::default();
    
    // Use the default options for metadata reader
    let metadata_opts = MetadataOptions::default();
    
    // Use the default options for the decoder
    let decoder_opts = DecoderOptions::default();
    
    // Probe the media source
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .map_err(|e| AppError::WhisperError(format!("Failed to probe media format: {}", e)))?;
    
    // Get the format reader
    let mut format = probed.format;
    
    // Find the first audio track
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| AppError::WhisperError("No audio track found".to_string()))?;
    
    // Create a decoder for the track
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .map_err(|e| AppError::WhisperError(format!("Failed to create decoder: {}", e)))?;
    
    // Store the track identifier
    let track_id = track.id;
    
    // Create a sample buffer
    let mut sample_buf = None;
    
    // Create a buffer for the decoded audio
    let mut audio_data = Vec::new();
    
    // Decode the audio
    loop {
        // Get the next packet from the format reader
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(ref e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(AppError::WhisperError(format!("Error reading packet: {}", e))),
        };
        
        // Skip packets from other tracks
        if packet.track_id() != track_id {
            continue;
        }
        
        // Decode the packet
        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!("Error decoding packet: {}", e);
                continue;
            }
        };
        
        // Get the audio buffer specification
        let spec = *decoded.spec();
        
        // Create a sample buffer if needed
        if sample_buf.is_none() {
            sample_buf = Some(SampleBuffer::<f32>::new(decoded.capacity() as u64, spec));
        }
        
        // Copy the decoded audio to the sample buffer
        if let Some(buf) = &mut sample_buf {
            buf.copy_planar_ref(decoded);
            
            // Convert the audio to f32 samples and add to our buffer
            audio_data.extend_from_slice(buf.samples());
        }
    }
    
    // If we have a stereo file, convert to mono by averaging channels
    let spec = decoder.codec_params();
    if spec.channels.unwrap().count() > 1 {
        let channels = spec.channels.unwrap().count() as usize;
        let samples_per_channel = audio_data.len() / channels;
        let mut mono_data = Vec::with_capacity(samples_per_channel);
        
        for i in 0..samples_per_channel {
            let mut sample_sum = 0.0;
            for c in 0..channels {
                sample_sum += audio_data[i + c * samples_per_channel];
            }
            mono_data.push(sample_sum / channels as f32);
        }
        
        audio_data = mono_data;
    }
    
    // Resample to 16kHz if needed
    let sample_rate = spec.sample_rate.unwrap_or(16000);
    if sample_rate != 16000 {
        // Simple resampling - in a real app you'd want to use a proper resampler
        let ratio = 16000.0 / sample_rate as f32;
        let new_len = (audio_data.len() as f32 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_len);
        
        for i in 0..new_len {
            let src_idx = (i as f32 / ratio) as usize;
            if src_idx < audio_data.len() {
                resampled.push(audio_data[src_idx]);
            } else {
                break;
            }
        }
        
        audio_data = resampled;
    }
    
    Ok(audio_data)
} 