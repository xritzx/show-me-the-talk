use std::thread;
use std::time::Duration;

/// Simulates Cmd+V by creating CGEvent keyboard events.
/// Requires Accessibility permissions on macOS.
pub fn simulate_paste() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        const KVK_ANSI_V: u16 = 9;

        let source = core_graphics::event_source::CGEventSource::new(
            core_graphics::event_source::CGEventSourceStateID::HIDSystemState,
        )
        .map_err(|_| "Failed to create event source".to_string())?;

        let key_down = core_graphics::event::CGEvent::new_keyboard_event(
            source.clone(),
            KVK_ANSI_V,
            true,
        )
        .map_err(|_| "Failed to create key down event".to_string())?;

        key_down.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);

        let key_up = core_graphics::event::CGEvent::new_keyboard_event(
            source,
            KVK_ANSI_V,
            false,
        )
        .map_err(|_| "Failed to create key up event".to_string())?;

        key_up.set_flags(core_graphics::event::CGEventFlags::CGEventFlagCommand);

        key_down.post(core_graphics::event::CGEventTapLocation::HID);
        thread::sleep(Duration::from_millis(50));
        key_up.post(core_graphics::event::CGEventTapLocation::HID);

        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        Err("Paste simulation is only supported on macOS".to_string())
    }
}
