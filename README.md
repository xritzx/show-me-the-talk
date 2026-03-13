# Show Me The Talk

Local voice-to-text for macOS. Hold a hotkey, speak, release — transcribed text is pasted instantly. Powered by Whisper.cpp with Metal acceleration. No cloud, no API keys, model ships with the app.

## Usage

1. Press and hold **Cmd + Option + /**
2. Speak
3. Release — text is pasted into your active field

## Build

Requires macOS 14+, Rust 1.85+, Node.js 22+, and cmake (`brew install cmake`).

```bash
npm install
npm run download:model
npm run tauri:build
```

The Whisper model (`ggml-base.en.bin`, ~141MB) is downloaded by `download:model` and bundled into the app.

## Permissions

- **Microphone** — prompted on first use
- **Accessibility** — required for auto-paste. Add the app in System Settings > Privacy & Security > Accessibility.

## License

MIT
