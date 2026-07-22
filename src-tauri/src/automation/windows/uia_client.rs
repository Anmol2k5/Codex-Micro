use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Com::*,
    Win32::UI::Accessibility::*,
};

pub struct UiaClient {
    automation: IUIAutomation,
}

#[allow(dead_code)]
impl UiaClient {
    pub fn new() -> Result<Self> {
        unsafe {
            let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
            Ok(Self { automation })
        }
    }

    pub fn element_from_handle(&self, hwnd: HWND) -> Result<IUIAutomationElement> {
        unsafe { self.automation.ElementFromHandle(hwnd) }
    }

    pub fn create_true_condition(&self) -> Result<IUIAutomationCondition> {
        unsafe { self.automation.CreateTrueCondition() }
    }

    pub fn get_control_view_walker(&self) -> Result<IUIAutomationTreeWalker> {
        unsafe { self.automation.ControlViewWalker() }
    }

    pub fn get_raw_view_walker(&self) -> Result<IUIAutomationTreeWalker> {
        unsafe { self.automation.RawViewWalker() }
    }
}
