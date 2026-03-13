interface TranscriptionResultProps {
  text: string
}

export const TranscriptionResult = ({ text }: TranscriptionResultProps) => {
  return (
    <div className="bg-[var(--color-surface)] border border-[var(--color-border)] rounded-lg p-3">
      <div className="flex items-center justify-between mb-1.5">
        <h2 className="text-xs font-semibold text-[var(--color-primary-light)]">
          Last Transcription
        </h2>
        <span className="text-[10px] text-[var(--color-text-muted)]">
          Auto-pasted
        </span>
      </div>
      <p className="text-xs text-[var(--color-text)] leading-relaxed select-text">
        {text}
      </p>
    </div>
  )
}
