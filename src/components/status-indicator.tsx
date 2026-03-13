type AppStatus = 'idle' | 'recording' | 'transcribing' | 'error'

interface StatusIndicatorProps {
  status: AppStatus
}

const STATUS_CONFIG: Record<AppStatus, { label: string; color: string; dotClass: string }> = {
  idle: {
    label: 'Ready',
    color: 'bg-[var(--color-success)]',
    dotClass: '',
  },
  recording: {
    label: 'Recording...',
    color: 'bg-[var(--color-recording)]',
    dotClass: 'animate-pulse-recording',
  },
  transcribing: {
    label: 'Transcribing...',
    color: 'bg-[var(--color-transcribing)]',
    dotClass: 'animate-spin-slow',
  },
  error: {
    label: 'Error',
    color: 'bg-red-500',
    dotClass: '',
  },
}

export const StatusIndicator = ({ status }: StatusIndicatorProps) => {
  const config = STATUS_CONFIG[status]

  return (
    <div className="flex items-center gap-2">
      <div className={`w-2 h-2 rounded-full ${config.color} ${config.dotClass}`} />
      <span className="text-xs text-[var(--color-text-muted)]">{config.label}</span>
    </div>
  )
}
