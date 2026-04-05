# Show Me The Talk

Local voice-to-text for macOS. Hold a hotkey, speak, release — transcribed text is pasted instantly. Powered by Whisper.cpp with Metal acceleration. Optional AI text refinement via a bundled Qwen 2.5 1.5B model. No cloud, no API keys, everything runs locally.

## Usage

1. Press and hold **Cmd + Option + /**
2. Speak
3. Release — text is pasted into your active field

### AI Text Refinement

Toggle **AI text refinement** in the settings panel to post-process transcriptions with the bundled Qwen 2.5 LLM. This improves grammar and clarity while keeping the meaning intact. An optional **SQL formatting** toggle is also available for SQL-heavy transcriptions.

## Build

Requires macOS 14+, Rust 1.85+, Node.js 22+, and cmake (`brew install cmake`).

```bash
npm install
npm run download:model
npm run download:llm
npm run tauri:build
```

- **Whisper model** (`ggml-base.en.bin`, ~141MB) — speech-to-text
- **Qwen 2.5 LLM** (`Qwen2.5-1.5B-Instruct-Q4_0.gguf`, ~940MB) — AI text refinement, Q4_0 with online weight repacking for Apple Silicon (Apache-2.0 license)

Both models are downloaded on demand and bundled into the app.

## Permissions

- **Microphone** — prompted on first use
- **Accessibility** — required for auto-paste. Add the app in System Settings > Privacy & Security > Accessibility.

## License

MIT
