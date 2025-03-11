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

// Load the audio file and convert it to the required format
let audio_data = load_audio_file(audio_path)?;

let mut state = ctx.create_state()
    .map_err(|e| AppError::WhisperError(format!("Failed to create Whisper state: {}", e)))?;

state.full(params, &audio_data)
    .map_err(|e| AppError::WhisperError(format!("Failed to run Whisper inference: {}", e)))?;

// Add this new function to load and convert audio
fn load_audio_file(path: &Path) -> Result<Vec<f32>, AppError> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;
    
    let file = std::fs::File::open(path)
        .map_err(|e| AppError::IoError(e))?;
    
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .map_err(|e| AppError::WhisperError(format!("Failed to probe audio format: {}", e)))?;
    
    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| AppError::WhisperError("No supported audio track found".to_string()))?;
    
    let mut decoder = symphonia::default::get_codecs()
        .make_decoder(&track.codec_params, &decoder_opts)
        .map_err(|e| AppError::WhisperError(format!("Failed to create decoder: {}", e)))?;
    
    let mut sample_buf = None;
    let mut audio_data = Vec::new();
    
    while let Ok(packet) = format.next_packet() {
        let decoded = decoder
            .decode(&packet)
            .map_err(|e| AppError::WhisperError(format!("Failed to decode audio: {}", e)))?;
        
        if sample_buf.is_none() {
            sample_buf = Some(SampleBuffer::<f32>::new(
                decoded.capacity() as u64,
                decoded.spec(),
            ));
        }
        
        if let Some(buf) = &mut sample_buf {
            buf.copy_interleaved_ref(decoded);
            audio_data.extend_from_slice(buf.samples());
        }
    }
    
    Ok(audio_data)
}

pub fn transcribe_audio(audio_path: &str) -> Result<String, AppError> {
    // Load Whisper model
    let model_path = download_whisper_model()?;
    let ctx = WhisperContext::new_with_params(&model_path, whisper_rs::WhisperContextParameters::default())
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
    
    // Load the audio file and convert it to the required format
    let audio_data = load_audio_file(audio_path)?;
    
    let mut state = ctx.create_state()
        .map_err(|e| AppError::WhisperError(format!("Failed to create Whisper state: {}", e)))?;
    
    state.full(params, &audio_data)
        .map_err(|e| AppError::WhisperError(format!("Failed to run Whisper inference: {}", e)))?;

    Ok(String::new())
} 