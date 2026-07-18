//! Conservative Windows target discovery and focusing.
//!
//! This module intentionally limits itself to top-level window discovery and focus. Codex-specific
//! controls remain behind UI Automation selectors that must be verified on a real Windows install.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetWindow {
    pub hwnd: isize,
    pub process_id: u32,
    pub process_name: String,
    pub window_title: String,
}

#[derive(Debug, Clone)]
pub struct TargetAppLocator {
    process_candidates: Vec<String>,
    title_candidates: Vec<String>,
}

impl Default for TargetAppLocator {
    fn default() -> Self {
        Self {
            process_candidates: vec!["ChatGPT.exe".into(), "Codex.exe".into()],
            title_candidates: vec!["ChatGPT".into(), "Codex".into()],
        }
    }
}

impl TargetAppLocator {
    pub fn new(process_candidates: Vec<String>) -> Self {
        Self {
            process_candidates,
            title_candidates: vec!["ChatGPT".into(), "Codex".into()],
        }
    }

    pub fn process_candidates(&self) -> &[String] {
        &self.process_candidates
    }

    pub fn title_candidates(&self) -> &[String] {
        &self.title_candidates
    }

    #[cfg(windows)]
    pub fn locate(&self) -> Option<TargetWindow> {
        windows_impl::locate(self)
    }

    #[cfg(not(windows))]
    pub fn locate(&self) -> Option<TargetWindow> {
        None
    }

    #[cfg(windows)]
    pub fn focus(&self, target: &TargetWindow) -> bool {
        windows_impl::focus(target.hwnd)
    }

    #[cfg(not(windows))]
    pub fn focus(&self, _target: &TargetWindow) -> bool {
        false
    }

    #[cfg(windows)]
    pub fn is_foreground(&self, target: &TargetWindow) -> bool {
        windows_impl::is_foreground(target.hwnd)
    }

    #[cfg(not(windows))]
    pub fn is_foreground(&self, _target: &TargetWindow) -> bool {
        false
    }
}

#[cfg(windows)]
#[allow(non_snake_case)]
mod windows_impl {
    use super::{TargetAppLocator, TargetWindow};
    use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::Path};

    type Hwnd = isize;
    type Handle = isize;
    type Bool = i32;
    type Lparam = isize;

    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    const SW_RESTORE: i32 = 9;

    #[link(name = "user32")]
    extern "system" {
        fn EnumWindows(callback: Option<unsafe extern "system" fn(Hwnd, Lparam) -> Bool>, lparam: Lparam) -> Bool;
        fn IsWindowVisible(hwnd: Hwnd) -> Bool;
        fn GetWindowTextLengthW(hwnd: Hwnd) -> i32;
        fn GetWindowTextW(hwnd: Hwnd, text: *mut u16, max_count: i32) -> i32;
        fn GetWindowThreadProcessId(hwnd: Hwnd, process_id: *mut u32) -> u32;
        fn GetForegroundWindow() -> Hwnd;
        fn SetForegroundWindow(hwnd: Hwnd) -> Bool;
        fn ShowWindow(hwnd: Hwnd, command: i32) -> Bool;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn OpenProcess(access: u32, inherit_handle: Bool, process_id: u32) -> Handle;
        fn QueryFullProcessImageNameW(
            process: Handle,
            flags: u32,
            image_name: *mut u16,
            size: *mut u32,
        ) -> Bool;
        fn CloseHandle(handle: Handle) -> Bool;
    }

    struct SearchContext {
        process_candidates: Vec<String>,
        title_candidates: Vec<String>,
        result: Option<TargetWindow>,
    }

    pub fn locate(locator: &TargetAppLocator) -> Option<TargetWindow> {
        let mut context = SearchContext {
            process_candidates: locator.process_candidates.clone(),
            title_candidates: locator.title_candidates.clone(),
            result: None,
        };
        unsafe {
            EnumWindows(
                Some(enum_window),
                (&mut context as *mut SearchContext) as isize,
            );
        }
        context.result
    }

    unsafe extern "system" fn enum_window(hwnd: Hwnd, lparam: Lparam) -> Bool {
        let context = &mut *(lparam as *mut SearchContext);
        if context.result.is_some() || IsWindowVisible(hwnd) == 0 {
            return 1;
        }

        let title = window_title(hwnd);
        if title.is_empty() {
            return 1;
        }

        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, &mut process_id);
        let process_name = process_name(process_id).unwrap_or_default();

        let process_matches = context
            .process_candidates
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(&process_name));
        let title_matches = context
            .title_candidates
            .iter()
            .any(|candidate| title.to_lowercase().contains(&candidate.to_lowercase()));

        let matches_target = if process_name.is_empty() { title_matches } else { process_matches };

        if matches_target {
            context.result = Some(TargetWindow {
                hwnd,
                process_id,
                process_name,
                window_title: title,
            });
            return 0;
        }

        1
    }

    unsafe fn window_title(hwnd: Hwnd) -> String {
        let length = GetWindowTextLengthW(hwnd);
        if length <= 0 {
            return String::new();
        }
        let mut buffer = vec![0u16; length as usize + 1];
        let written = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
        String::from_utf16_lossy(&buffer[..written.max(0) as usize])
    }

    unsafe fn process_name(process_id: u32) -> Option<String> {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id);
        if handle == 0 {
            return None;
        }

        let mut buffer = vec![0u16; 32768];
        let mut size = buffer.len() as u32;
        let success = QueryFullProcessImageNameW(handle, 0, buffer.as_mut_ptr(), &mut size);
        CloseHandle(handle);
        if success == 0 || size == 0 {
            return None;
        }

        let path = OsString::from_wide(&buffer[..size as usize]);
        Path::new(&path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
    }

    pub fn focus(hwnd: Hwnd) -> bool {
        unsafe {
            ShowWindow(hwnd, SW_RESTORE);
            SetForegroundWindow(hwnd) != 0
        }
    }

    pub fn is_foreground(hwnd: Hwnd) -> bool {
        unsafe { GetForegroundWindow() == hwnd }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_candidates_cover_current_and_migration_names() {
        let locator = TargetAppLocator::default();
        assert!(locator.process_candidates().iter().any(|name| name == "ChatGPT.exe"));
        assert!(locator.process_candidates().iter().any(|name| name == "Codex.exe"));
    }

    #[test]
    fn default_title_candidates_allow_chatgpt_and_codex_windows() {
        let locator = TargetAppLocator::default();
        assert!(locator.title_candidates().iter().any(|name| name == "ChatGPT"));
        assert!(locator.title_candidates().iter().any(|name| name == "Codex"));
    }
}
