use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::DataExchange::*,
    Win32::System::Memory::*,
    Win32::UI::Accessibility::IUIAutomationElement,
    Win32::UI::Input::KeyboardAndMouse::*,
};

#[allow(dead_code)]
fn set_clipboard_text(text: &str) {
    unsafe {
        if OpenClipboard(None).is_ok() {
            let _ = EmptyClipboard();
            let utf16: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
            let size = utf16.len() * 2;
            if let Ok(handle) = GlobalAlloc(GMEM_MOVEABLE, size) {
                let ptr = GlobalLock(handle);
                if !ptr.is_null() {
                    std::ptr::copy_nonoverlapping(utf16.as_ptr(), ptr as *mut u16, utf16.len());
                    let _ = GlobalUnlock(handle);
                    // CF_UNICODETEXT is constant 13
                    let _ = SetClipboardData(13, Some(HANDLE(handle.0)));
                }
            }
            let _ = CloseClipboard();
        }
    }
}

#[allow(dead_code)]
fn send_keyboard_inputs(inputs: &[INPUT]) {
    unsafe {
        let size = std::mem::size_of::<INPUT>() as i32;
        SendInput(inputs, size);
    }
}

#[allow(dead_code)]
fn make_key_input(vk: VIRTUAL_KEY, is_up: bool) -> INPUT {
    let mut input = INPUT::default();
    input.r#type = INPUT_KEYBOARD;
    input.Anonymous.ki = KEYBDINPUT {
        wVk: vk,
        wScan: 0,
        dwFlags: if is_up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) },
        time: 0,
        dwExtraInfo: 0,
    };
    input
}

#[allow(dead_code)]
pub fn focus_and_paste(element: &IUIAutomationElement, text: &str) -> Result<()> {
    unsafe {
        element.SetFocus()?;
    }
    std::thread::sleep(std::time::Duration::from_millis(150));

    set_clipboard_text(text);

    // Ctrl Down, V Down, V Up, Ctrl Up
    let inputs = [
        make_key_input(VK_CONTROL, false),
        make_key_input(VK_V, false),
        make_key_input(VK_V, true),
        make_key_input(VK_CONTROL, true),
    ];
    send_keyboard_inputs(&inputs);

    Ok(())
}

#[allow(dead_code)]
pub fn trigger_dictation(element: &IUIAutomationElement) -> Result<()> {
    unsafe {
        element.SetFocus()?;
    }
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Win Down, H Down, H Up, Win Up
    // VK_LWIN = 0x5B
    let inputs = [
        make_key_input(VK_LWIN, false),
        make_key_input(VK_H, false),
        make_key_input(VK_H, true),
        make_key_input(VK_LWIN, true),
    ];
    send_keyboard_inputs(&inputs);

    Ok(())
}
