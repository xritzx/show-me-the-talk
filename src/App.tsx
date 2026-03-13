import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { StatusIndicator } from './components/status-indicator'
import { Instructions } from './components/instructions'
import { TranscriptionResult } from './components/transcription-result'

type AppStatus = 'idle' | 'recording' | 'transcribing' | 'error'

interface TranscriptionEvent {
  text: string
}

export const App = () => {
  const [status, setStatus] = useState<AppStatus>('idle')
  const [lastResult, setLastResult] = useState<string>('')
  const [error, setError] = useState<string>('')

  useEffect(() => {
    const unlisteners: Array<() => void> = []

    const setup = async () => {
      unlisteners.push(
        await listen('recording-started', () => {
          setStatus('recording')
          setError('')
        })
      )
      unlisteners.push(
        await listen('recording-stopped', () => {
          setStatus('transcribing')
        })
      )
      unlisteners.push(
        await listen<TranscriptionEvent>('transcription-result', (event) => {
          setStatus('idle')
          setLastResult(event.payload.text)
        })
      )
      unlisteners.push(
        await listen<string>('transcription-error', (event) => {
          setStatus('error')
          setError(event.payload)
          setTimeout(() => setStatus('idle'), 3000)
        })
      )
    }

    setup()
    return () => unlisteners.forEach((fn) => fn())
  }, [])

  return (
    <div className="flex flex-col h-screen bg-[var(--color-bg)] p-4">
      <header className="flex items-center justify-between mb-4">
        <h1 className="text-sm font-semibold text-[var(--color-text)]">
          Show Me The Talk
        </h1>
        <StatusIndicator status={status} />
      </header>

      <div className="flex-1 flex flex-col gap-3 overflow-hidden">
        {lastResult && <TranscriptionResult text={lastResult} />}
        {error && (
          <div className="px-3 py-2 bg-red-900/30 border border-red-700/50 rounded-lg text-xs text-red-300">
            {error}
          </div>
        )}
        <Instructions status={status} />
      </div>

      <footer className="mt-3 text-center text-[10px] text-[var(--color-text-muted)]">
        Powered by Whisper.cpp &middot; All processing is local
      </footer>
    </div>
  )
}
