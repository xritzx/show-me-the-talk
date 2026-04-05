type AppStatus = 'idle' | 'recording' | 'transcribing' | 'refining' | 'error'

interface StatusIndicatorProps {
  status: AppStatus
}

const STATUS_CONFIG: Record<AppStatus, { label: string; color: string; glow: string; animated: boolean }> = {
  idle: {
    label: 'Ready',
    color: 'bg-emerald-400',
    glow: 'shadow-[0_0_6px_rgba(52,211,153,0.6)]',
    animated: false,
  },
  recording: {
    label: 'Recording',
    color: 'bg-red-400',
    glow: 'shadow-[0_0_6px_rgba(248,113,113,0.6)]',
    animated: true,
  },
  transcribing: {
    label: 'Transcribing',
    color: 'bg-amber-400',
    glow: 'shadow-[0_0_6px_rgba(251,191,36,0.6)]',
    animated: true,
  },
  refining: {
    label: 'Refining',
    color: 'bg-violet-400',
    glow: 'shadow-[0_0_6px_rgba(167,139,250,0.6)]',
    animated: true,
  },
  error: {
    label: 'Error',
    color: 'bg-red-500',
    glow: '',
    animated: false,
  },
}

export const StatusIndicator = ({ status }: StatusIndicatorProps) => {
  const config = STATUS_CONFIG[status]

  return (
    <div className="flex items-center gap-1.5">
      <div
        className={`w-2 h-2 rounded-full ${config.color} ${config.glow} ${
          config.animated ? 'animate-pulse-recording' : ''
        }`}
      />
      <span className="text-[10px] text-white/40 font-medium">{config.label}</span>
    </div>
  )
}
