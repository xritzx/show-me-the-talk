type AppStatus = 'idle' | 'recording' | 'transcribing' | 'error'

interface InstructionsProps {
  status: AppStatus
}

export const Instructions = ({ status }: InstructionsProps) => {
  return (
    <div className="bg-[var(--color-surface)] border border-[var(--color-border)] rounded-lg p-3">
      <h2 className="text-xs font-semibold mb-2 text-[var(--color-primary-light)]">
        How to use
      </h2>
      <ol className="text-xs text-[var(--color-text-muted)] space-y-1.5 list-decimal list-inside">
        <li>
          Click on any text field in any app
        </li>
        <li>
          Hold <Kbd>⌘</Kbd> + <Kbd>⌥</Kbd> + <Kbd>/</Kbd> to start recording
        </li>
        <li>
          Speak clearly into your microphone
        </li>
        <li>
          Release the keys — text will be transcribed and pasted
        </li>
      </ol>

      {status === 'recording' && (
        <div className="mt-3 flex items-center gap-2 text-xs text-[var(--color-recording)]">
          <div className="w-2 h-2 rounded-full bg-[var(--color-recording)] animate-pulse-recording" />
          Listening... release keys when done
        </div>
      )}

      {status === 'transcribing' && (
        <div className="mt-3 flex items-center gap-2 text-xs text-[var(--color-transcribing)]">
          <svg className="w-3 h-3 animate-spin-slow" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
            <circle cx="12" cy="12" r="10" strokeDasharray="31.4 31.4" strokeLinecap="round" />
          </svg>
          Processing your speech...
        </div>
      )}
    </div>
  )
}

const Kbd = ({ children }: { children: React.ReactNode }) => (
  <kbd className="inline-block px-1.5 py-0.5 bg-[var(--color-bg)] border border-[var(--color-border)] rounded text-[10px] font-mono text-[var(--color-text)]">
    {children}
  </kbd>
)
