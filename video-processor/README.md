# Video Processor

A desktop application for processing video files with automatic transcription and intelligent cutting.

## Features

- Extract audio from video files (MP4, AVI, MOV)
- Transcribe speech to text with timestamps using Whisper
- Analyze transcripts to determine strategic cut points
- Process videos by trimming at the correct points
- Apply zoom effects on faces during cuts (optional)
- Fully offline operation - no API calls required

## Prerequisites

Before running the application, make sure you have the following installed:

1. **FFmpeg**: Required for audio extraction and video processing
   - Install on macOS: `brew install ffmpeg`
   - Install on Windows: [Download from FFmpeg website](https://ffmpeg.org/download.html)
   - Install on Linux: `sudo apt install ffmpeg`

2. **Python with MoviePy**: Required for video effects and processing
   - Install Python: [python.org](https://www.python.org/downloads/)
   - Install MoviePy: `pip install moviepy`

## Development

This application is built with:

- [Tauri](https://tauri.app/) - Desktop application framework
- [Rust](https://www.rust-lang.org/) - Backend language
- [React](https://reactjs.org/) - Frontend framework
- [whisper-rs](https://github.com/tazz4843/whisper-rs) - Rust bindings for Whisper speech recognition
- [FFmpeg](https://ffmpeg.org/) - Audio/video processing

### Setup Development Environment

1. Clone the repository
2. Install dependencies:
   ```
   npm install
   ```
3. Run in development mode:
   ```
   npm run tauri dev
   ```

### Building for Production

To build the application for production:

```
npm run tauri build
```

This will create platform-specific installers in the `src-tauri/target/release` directory.

## How It Works

1. **Video Selection**: User selects a video file through the application.
2. **Audio Extraction**: FFmpeg extracts the audio track from the video.
3. **Transcription**: Whisper processes the audio to generate a transcript with timestamps.
4. **Analysis**: The transcript is analyzed to find natural breaks and pauses.
5. **Processing**: MoviePy trims the video at the determined cut points and applies effects if selected.
6. **Output**: The processed video is saved to the user's chosen location.

## License

MIT
