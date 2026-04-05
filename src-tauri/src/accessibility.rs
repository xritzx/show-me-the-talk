/// macOS Accessibility permission check/prompt via raw CoreFoundation FFI.
/// Required for simulating keyboard events (Cmd+V paste).

#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::c_void;
    use std::ptr;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXIsProcessTrusted() -> u8;
        fn AXIsProcessTrustedWithOptions(options: *const c_void) -> u8;
    }

    extern "C" {
        fn CFStringCreateWithCString(
            allocator: *const c_void,
            c_str: *const u8,
            encoding: u32,
        ) -> *const c_void;
        fn CFDictionaryCreate(
            allocator: *const c_void,
            keys: *const *const c_void,
            values: *const *const c_void,
            count: isize,
            key_callbacks: *const c_void,
            value_callbacks: *const c_void,
        ) -> *const c_void;
        fn CFRelease(cf: *const c_void);
        static kCFBooleanTrue: *const c_void;
        static kCFTypeDictionaryKeyCallBacks: u8;
        static kCFTypeDictionaryValueCallBacks: u8;
    }

    const CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;

    pub fn is_trusted() -> bool {
        unsafe { AXIsProcessTrusted() != 0 }
    }

    /// Checks trust status and shows the native macOS prompt dialog if not trusted.
    pub fn prompt_for_trust() -> bool {
        unsafe {
            let key = CFStringCreateWithCString(
                ptr::null(),
                b"AXTrustedCheckOptionPrompt\0".as_ptr(),
                CF_STRING_ENCODING_UTF8,
            );
            if key.is_null() {
                return is_trusted();
            }

            let keys = [key];
            let values = [kCFBooleanTrue];

            let options = CFDictionaryCreate(
                ptr::null(),
                keys.as_ptr(),
                values.as_ptr(),
                1,
                &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
                &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
            );

            let result = AXIsProcessTrustedWithOptions(options);

            CFRelease(key);
            if !options.is_null() {
                CFRelease(options);
            }

            result != 0
        }
    }
}

pub fn is_trusted() -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::is_trusted()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

pub fn prompt_for_trust() -> bool {
    #[cfg(target_os = "macos")]
    {
        macos::prompt_for_trust()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}
