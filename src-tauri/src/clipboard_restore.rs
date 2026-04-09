use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Returns `Some(text)` when the system clipboard holds readable UTF-8; `None` on error (e.g. image-only).
pub fn snapshot_text_clipboard(handle: &AppHandle) -> Option<String> {
    handle.clipboard().read_text().ok()
}

/// Writes the snapshot back when present. Logs a warning on failure but never panics.
pub fn restore_text_clipboard(handle: &AppHandle, snapshot: &Option<String>) {
    if let Some(s) = snapshot {
        if let Err(e) = handle.clipboard().write_text(s.as_str()) {
            log::warn!("Failed to restore clipboard: {}", e);
        }
    }
}
