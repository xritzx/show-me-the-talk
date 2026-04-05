import { useEffect, useState, useCallback } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { StatusIndicator } from './components/status-indicator'
import { Instructions } from './components/instructions'
import { TranscriptionResult } from './components/transcription-result'

type AppStatus = 'idle' | 'recording' | 'transcribing' | 'refining' | 'error'

interface TranscriptionEvent {
  raw: string
  refined: string | null
}

interface AppSettings {
  llm_enabled: boolean
  include_sql_instructions: boolean
}

export const App = () => {
  const [status, setStatus] = useState<AppStatus>('idle')
  const [rawText, setRawText] = useState<string>('')
  const [refinedText, setRefinedText] = useState<string | null>(null)
  const [error, setError] = useState<string>('')
  const [accessibilityGranted, setAccessibilityGranted] = useState(true)
  const [settings, setSettings] = useState<AppSettings>({
    llm_enabled: false,
    include_sql_instructions: false,
  })

  useEffect(() => {
    invoke<AppSettings>('get_settings')
      .then(setSettings)
      .catch((e) => console.error('Failed to load settings:', e))
  }, [])

  useEffect(() => {
    const checkAccess = () => {
      invoke<boolean>('check_accessibility')
        .then((granted) => setAccessibilityGranted(granted))
        .catch(() => {})
    }
    checkAccess()
    const interval = setInterval(checkAccess, 3000)
    return () => clearInterval(interval)
  }, [])

  const updateSettings = useCallback(
    (patch: Partial<AppSettings>) => {
      const updated = { ...settings, ...patch }
      setSettings(updated)
      invoke('set_settings', { updated }).catch((e) =>
        console.error('Failed to save settings:', e)
      )
    },
    [settings]
  )

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
        await listen('llm-processing', () => {
          setStatus('refining')
        })
      )
      unlisteners.push(
        await listen<TranscriptionEvent>('transcription-result', (event) => {
          setStatus('idle')
          setRawText(event.payload.raw)
          setRefinedText(event.payload.refined)
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
    <div className="h-screen w-screen p-8">
      <div className="app-shell flex flex-col h-full rounded-2xl shadow-[0_8px_40px_rgba(0,0,0,0.5)] overflow-hidden">
        <TitleBar status={status} />

        <div className="flex-1 flex flex-col gap-2.5 overflow-y-auto p-8">
          {!accessibilityGranted && (
            <div className="panel px-3 py-2.5 rounded-xl text-xs border border-amber-500/20 bg-amber-500/5">
              <p className="font-medium text-amber-400 mb-1">Accessibility permission required</p>
              <p className="text-amber-400/70 leading-relaxed">
                Open <strong>System Settings &rarr; Privacy &amp; Security &rarr; Accessibility</strong> and
                enable <strong>Show Me The Talk</strong> to allow auto-paste.
              </p>
            </div>
          )}
          {rawText && <TranscriptionResult raw={rawText} refined={refinedText} />}
          {error && (
            <div className="panel px-3 py-2 rounded-xl text-xs text-red-400 border-red-500/20">
              {error}
            </div>
          )}
          <Instructions status={status} />
          <SettingsPanel settings={settings} onUpdate={updateSettings} />
        </div>

        <footer className="px-4 py-2.5 text-center text-[10px] text-slate-500 border-t border-white/5">
          Powered by Whisper.cpp{settings.llm_enabled ? ' + Qwen 2.5' : ''} &middot; All local
        </footer>
      </div>
    </div>
  )
}

const TitleBar = ({ status }: { status: AppStatus }) => {
  const win = getCurrentWindow()

  return (
    <div
      data-tauri-drag-region
      className="flex items-center justify-between px-4 py-3 border-b border-white/5 overflow-hidden shrink-0 cursor-default"
    >
      <div className="flex items-center gap-2.5" data-tauri-drag-region>
        <div className="w-6 h-6 rounded-lg bg-gradient-to-br from-violet-500 to-purple-600 flex items-center justify-center shadow-[0_0_12px_rgba(124,58,237,0.35)]">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round">
            <path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z" />
            <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
            <line x1="12" x2="12" y1="19" y2="22" />
          </svg>
        </div>
        <span className="text-[13px] font-semibold text-slate-200 tracking-wide" data-tauri-drag-region>
          Show Me The Talk
        </span>
      </div>

      <div className="flex items-center gap-3">
        <StatusIndicator status={status} />
        <div className="flex items-center gap-1.5 ml-1">
          <button
            onClick={() => win.minimize()}
            className="w-3 h-3 rounded-full bg-white/8 hover:bg-amber-500 transition-colors duration-150"
            title="Minimize"
          />
          <button
            onClick={() => win.hide()}
            className="w-3 h-3 rounded-full bg-white/8 hover:bg-red-500 transition-colors duration-150"
            title="Close"
          />
        </div>
      </div>
    </div>
  )
}

interface SettingsPanelProps {
  settings: AppSettings
  onUpdate: (patch: Partial<AppSettings>) => void
}

const SettingsPanel = ({ settings, onUpdate }: SettingsPanelProps) => (
  <div className="panel rounded-xl p-3">
    <h2 className="text-[11px] font-semibold mb-2.5 text-slate-400 uppercase tracking-wider">
      Settings
    </h2>
    <div className="space-y-2.5">
      <Toggle
        label="AI text refinement"
        checked={settings.llm_enabled}
        onChange={(v) => onUpdate({ llm_enabled: v })}
      />
      {settings.llm_enabled && (
        <Toggle
          label="SQL formatting"
          checked={settings.include_sql_instructions}
          onChange={(v) => onUpdate({ include_sql_instructions: v })}
        />
      )}
    </div>
  </div>
)

interface ToggleProps {
  label: string
  checked: boolean
  onChange: (value: boolean) => void
}

const Toggle = ({ label, checked, onChange }: ToggleProps) => (
  <label className="flex items-center justify-between cursor-pointer group">
    <span className="text-xs text-slate-400 group-hover:text-slate-300 transition-colors">
      {label}
    </span>
    <button
      role="switch"
      aria-checked={checked}
      onClick={() => onChange(!checked)}
      className={`relative inline-flex h-5 w-9 items-center rounded-full transition-all duration-200 ${
        checked
          ? 'bg-gradient-to-r from-violet-600 to-purple-500 shadow-[0_0_10px_rgba(124,58,237,0.4)]'
          : 'bg-white/8 border border-white/6'
      }`}
    >
      <span
        className={`inline-block h-3.5 w-3.5 rounded-full bg-white shadow-sm transition-transform duration-200 ${
          checked ? 'translate-x-4' : 'translate-x-0.5'
        }`}
      />
    </button>
  </label>
)
