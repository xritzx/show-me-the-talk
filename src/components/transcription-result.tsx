interface TranscriptionResultProps {
  raw: string
  refined: string | null
}

export const TranscriptionResult = ({ raw, refined }: TranscriptionResultProps) => {
  return (
    <div className="panel rounded-xl overflow-hidden">
      <div className="h-[2px] bg-gradient-to-r from-violet-500/50 via-purple-500/30 to-transparent" />
      <div className="p-3 space-y-3 overflow-y-auto">
        <div>
          <div className="flex items-center justify-between mb-1.5">
            <h2 className="text-[11px] font-semibold text-slate-400 uppercase tracking-wider">
              {refined ? 'Raw Transcription' : 'Transcription'}
            </h2>
            {!refined && (
              <PastedBadge />
            )}
          </div>
          <div className="max-h-24 overflow-y-auto">
            <p className="text-xs text-slate-300/70 leading-relaxed select-text">
              {raw}
            </p>
          </div>
        </div>

        {refined && (
          <div>
            <div className="flex items-center justify-between mb-1.5">
              <h2 className="text-[11px] font-semibold text-violet-400 uppercase tracking-wider flex items-center gap-1.5">
                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
                </svg>
                AI Refined
              </h2>
              <PastedBadge />
            </div>
            <div className="max-h-28 overflow-y-auto panel-inset rounded-lg p-2">
              <p className="text-xs text-slate-200 leading-relaxed select-text">
                {refined}
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

const PastedBadge = () => (
  <span className="text-[10px] text-slate-500 flex items-center gap-1">
    <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <polyline points="20 6 9 17 4 12" />
    </svg>
    Pasted
  </span>
)
