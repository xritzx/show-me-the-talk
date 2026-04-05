type AppStatus = 'idle' | 'recording' | 'transcribing' | 'refining' | 'error'

interface InstructionsProps {
  status: AppStatus
}

export const Instructions = ({ status }: InstructionsProps) => {
  return (
    <div className="panel rounded-xl p-3">
      <h2 className="text-[11px] font-semibold mb-2.5 text-slate-400 uppercase tracking-wider">
        How to use
      </h2>
      <ol className="text-xs text-slate-400 space-y-2 list-none">
        <Step n={1}>Click on any text field in any app</Step>
        <Step n={2}>
          Hold <Kbd>⌘</Kbd> + <Kbd>⌥</Kbd> + <Kbd>/</Kbd> to record
        </Step>
        <Step n={3}>Speak clearly into your microphone</Step>
        <Step n={4}>Release — text is transcribed and pasted</Step>
      </ol>

      {status === 'recording' && (
        <StatusMessage color="text-red-400" dotColor="bg-red-400" animated>
          Listening... release keys when done
        </StatusMessage>
      )}

      {status === 'transcribing' && (
        <StatusMessage color="text-amber-400" dotColor="bg-amber-400" spinner>
          Processing your speech...
        </StatusMessage>
      )}

      {status === 'refining' && (
        <StatusMessage color="text-violet-400" dotColor="bg-violet-400" spinner>
          Refining text with AI...
        </StatusMessage>
      )}
    </div>
  )
}

const Step = ({ n, children }: { n: number; children: React.ReactNode }) => (
  <li className="flex items-start gap-2.5">
    <span className="shrink-0 w-[18px] h-[18px] rounded-md bg-white/5 border border-white/8 flex items-center justify-center text-[9px] font-bold text-slate-500 mt-0.5">
      {n}
    </span>
    <span className="leading-relaxed">{children}</span>
  </li>
)

const StatusMessage = ({
  color,
  dotColor,
  animated,
  spinner,
  children,
}: {
  color: string
  dotColor: string
  animated?: boolean
  spinner?: boolean
  children: React.ReactNode
}) => (
  <div className={`mt-3 flex items-center gap-2 text-xs ${color}`}>
    {spinner ? (
      <svg className="w-3 h-3 animate-spin-slow" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <circle cx="12" cy="12" r="10" strokeDasharray="31.4 31.4" strokeLinecap="round" />
      </svg>
    ) : (
      <div
        className={`w-2 h-2 rounded-full ${dotColor} ${
          animated ? 'animate-pulse-recording' : ''
        }`}
      />
    )}
    {children}
  </div>
)

const Kbd = ({ children }: { children: React.ReactNode }) => (
  <kbd className="inline-block px-1.5 py-0.5 bg-white/5 border border-white/8 rounded text-[10px] font-mono text-slate-300">
    {children}
  </kbd>
)
