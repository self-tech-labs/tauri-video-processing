import { useState, useEffect } from 'react'
import './App.css'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'

// Components
import VideoUploader from './components/VideoUploader'
import TranscriptViewer from './components/TranscriptViewer'
import ProcessingOptions from './components/ProcessingOptions'
import VideoPreview from './components/VideoPreview'
import ProgressBar from './components/ProgressBar'

// Types
import { CutPoint, VideoProcessingOptions, Transcript } from './types'

function App() {
  const [videoPath, setVideoPath] = useState<string | null>(null)
  const [audioPath, setAudioPath] = useState<string | null>(null)
  const [transcriptPath, setTranscriptPath] = useState<string | null>(null)
  const [transcript, setTranscript] = useState<Transcript | null>(null)
  const [cutPoints, setCutPoints] = useState<CutPoint[]>([])
  const [outputPath, setOutputPath] = useState<string | null>(null)
  const [processedVideoPath, setProcessedVideoPath] = useState<string | null>(null)
  const [currentStep, setCurrentStep] = useState<number>(0)
  const [progress, setProgress] = useState<number>(0)
  const [error, setError] = useState<string | null>(null)
  const [applyZoomEffects, setApplyZoomEffects] = useState<boolean>(false)

  // Steps in the process
  const steps = [
    'Select Video',
    'Extract Audio',
    'Transcribe Audio',
    'Analyze Transcript',
    'Process Video',
    'Done'
  ]

  // Handle video selection
  const handleVideoSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Video',
          extensions: ['mp4', 'avi', 'mov']
        }]
      })

      if (selected && !Array.isArray(selected)) {
        setVideoPath(selected)
        setCurrentStep(1)
      }
    } catch (err) {
      setError(`Error selecting video: ${err}`)
    }
  }

  // Extract audio from video
  const extractAudio = async () => {
    if (!videoPath) return

    try {
      setProgress(0)
      const progressInterval = setInterval(() => {
        setProgress(prev => Math.min(prev + 5, 90))
      }, 500)

      const result = await invoke<string>('extract_audio_from_video', { videoPath })
      setAudioPath(result)
      setCurrentStep(2)
      
      clearInterval(progressInterval)
      setProgress(100)
      
      // Automatically move to next step after a short delay
      setTimeout(() => {
        setProgress(0)
        setCurrentStep(2)
      }, 1000)
    } catch (err) {
      setError(`Error extracting audio: ${err}`)
    }
  }

  // Transcribe audio
  const transcribeAudio = async () => {
    if (!audioPath) return

    try {
      setProgress(0)
      const progressInterval = setInterval(() => {
        setProgress(prev => Math.min(prev + 2, 90))
      }, 500)

      const result = await invoke<string>('transcribe_audio_file', { audioPath })
      setTranscriptPath(result)
      
      // Load transcript content
      const transcriptContent = await fetch(`file://${result}`).then(res => res.json())
      setTranscript(transcriptContent)
      
      clearInterval(progressInterval)
      setProgress(100)
      
      // Automatically move to next step after a short delay
      setTimeout(() => {
        setProgress(0)
        setCurrentStep(3)
      }, 1000)
    } catch (err) {
      setError(`Error transcribing audio: ${err}`)
    }
  }

  // Analyze transcript for cut points
  const analyzeTranscript = async () => {
    if (!transcript) return

    // For now, we'll use a simple algorithm to find natural breaks
    const segments = transcript.segments
    const newCutPoints: CutPoint[] = []
    let currentStart = 0

    for (let i = 1; i < segments.length; i++) {
      const prevSegment = segments[i - 1]
      const currSegment = segments[i]
      
      // If there's a pause of more than 1 second, consider it a cut point
      const pauseDuration = currSegment.start - prevSegment.end
      if (pauseDuration > 1.0) {
        newCutPoints.push({
          startTime: currentStart,
          endTime: prevSegment.end,
          description: `Segment ${newCutPoints.length + 1}`
        })
        
        currentStart = currSegment.start
      }
    }
    
    // Add the final segment
    if (segments.length > 0) {
      const lastSegment = segments[segments.length - 1]
      newCutPoints.push({
        startTime: currentStart,
        endTime: lastSegment.end,
        description: `Segment ${newCutPoints.length + 1}`
      })
    }

    setCutPoints(newCutPoints)
    setCurrentStep(4)
  }

  // Process video
  const processVideo = async () => {
    if (!videoPath || !transcriptPath || cutPoints.length === 0) return

    try {
      // Ask user for output path
      const savePath = await save({
        filters: [{
          name: 'Video',
          extensions: ['mp4']
        }]
      })

      if (!savePath) return
      setOutputPath(savePath)

      setProgress(0)
      const progressInterval = setInterval(() => {
        setProgress(prev => Math.min(prev + 1, 90))
      }, 500)

      const options: VideoProcessingOptions = {
        outputPath: savePath,
        cutPoints: cutPoints,
        applyZoomEffects
      }

      const result = await invoke<string>('process_video_file', { 
        videoPath, 
        transcriptPath,
        options
      })
      
      setProcessedVideoPath(result)
      clearInterval(progressInterval)
      setProgress(100)
      
      // Move to final step
      setTimeout(() => {
        setCurrentStep(5)
      }, 1000)
    } catch (err) {
      setError(`Error processing video: ${err}`)
    }
  }

  // Update cut points
  const updateCutPoint = (index: number, cutPoint: CutPoint) => {
    const newCutPoints = [...cutPoints]
    newCutPoints[index] = cutPoint
    setCutPoints(newCutPoints)
  }

  // Add new cut point
  const addCutPoint = () => {
    if (!transcript || transcript.segments.length === 0) return
    
    const lastSegment = transcript.segments[transcript.segments.length - 1]
    const newCutPoint: CutPoint = {
      startTime: 0,
      endTime: lastSegment.end,
      description: `Segment ${cutPoints.length + 1}`
    }
    
    setCutPoints([...cutPoints, newCutPoint])
  }

  // Remove cut point
  const removeCutPoint = (index: number) => {
    const newCutPoints = cutPoints.filter((_, i) => i !== index)
    setCutPoints(newCutPoints)
  }

  // Effect to automatically proceed with steps
  useEffect(() => {
    if (currentStep === 1 && videoPath) {
      extractAudio()
    } else if (currentStep === 2 && audioPath) {
      transcribeAudio()
    }
  }, [currentStep, videoPath, audioPath])

  return (
    <div className="app-container">
      <header>
        <h1>Video Processor</h1>
        <div className="steps">
          {steps.map((step, index) => (
            <div 
              key={index} 
              className={`step ${index === currentStep ? 'active' : ''} ${index < currentStep ? 'completed' : ''}`}
            >
              {step}
            </div>
          ))}
        </div>
      </header>

      <main>
        {error && (
          <div className="error-message">
            <p>{error}</p>
            <button onClick={() => setError(null)}>Dismiss</button>
          </div>
        )}

        {currentStep === 0 && (
          <VideoUploader onVideoSelect={handleVideoSelect} />
        )}

        {currentStep > 0 && currentStep < 5 && progress > 0 && (
          <ProgressBar progress={progress} />
        )}

        {currentStep === 3 && transcript && (
          <TranscriptViewer transcript={transcript} />
        )}

        {currentStep === 4 && (
          <ProcessingOptions 
            cutPoints={cutPoints}
            updateCutPoint={updateCutPoint}
            addCutPoint={addCutPoint}
            removeCutPoint={removeCutPoint}
            applyZoomEffects={applyZoomEffects}
            setApplyZoomEffects={setApplyZoomEffects}
            onProcess={processVideo}
          />
        )}

        {currentStep === 5 && processedVideoPath && (
          <VideoPreview videoPath={processedVideoPath} />
        )}
      </main>

      <footer>
        <p>Powered by Tauri, Rust, and React</p>
      </footer>
    </div>
  )
}

export default App
