use windows::{
    core::*,
    Win32::UI::Accessibility::*,
};

#[allow(dead_code)]
pub fn invoke_element(element: &IUIAutomationElement) -> Result<()> {
    unsafe {
        // Pattern ID for InvokePattern is 10000 (UIA_InvokePatternId)
        let pattern_obj = element.GetCurrentPattern(UIA_InvokePatternId)?;
        let invoke_pattern: IUIAutomationInvokePattern = pattern_obj.cast()?;
        invoke_pattern.Invoke()
    }
}

#[allow(dead_code)]
pub fn select_element(element: &IUIAutomationElement) -> Result<()> {
    unsafe {
        // Pattern ID for SelectionItemPattern is 10010 (UIA_SelectionItemPatternId)
        let pattern_obj = element.GetCurrentPattern(UIA_SelectionItemPatternId)?;
        let selection_pattern: IUIAutomationSelectionItemPattern = pattern_obj.cast()?;
        selection_pattern.Select()
    }
}

#[allow(dead_code)]
pub fn toggle_element(element: &IUIAutomationElement) -> Result<()> {
    unsafe {
        // Pattern ID for TogglePattern is 10015 (UIA_TogglePatternId)
        let pattern_obj = element.GetCurrentPattern(UIA_TogglePatternId)?;
        let toggle_pattern: IUIAutomationTogglePattern = pattern_obj.cast()?;
        toggle_pattern.Toggle()
    }
}

#[allow(dead_code)]
pub fn expand_element(element: &IUIAutomationElement) -> Result<()> {
    unsafe {
        // Pattern ID for ExpandCollapsePattern is 10005 (UIA_ExpandCollapsePatternId)
        let pattern_obj = element.GetCurrentPattern(UIA_ExpandCollapsePatternId)?;
        let expand_pattern: IUIAutomationExpandCollapsePattern = pattern_obj.cast()?;
        expand_pattern.Expand()
    }
}

#[allow(dead_code)]
pub fn collapse_element(element: &IUIAutomationElement) -> Result<()> {
    unsafe {
        // Pattern ID for ExpandCollapsePattern is 10005 (UIA_ExpandCollapsePatternId)
        let pattern_obj = element.GetCurrentPattern(UIA_ExpandCollapsePatternId)?;
        let expand_pattern: IUIAutomationExpandCollapsePattern = pattern_obj.cast()?;
        expand_pattern.Collapse()
    }
}

#[allow(dead_code)]
pub fn set_element_value(element: &IUIAutomationElement, value: &str) -> Result<()> {
    unsafe {
        // Pattern ID for ValuePattern is 10002 (UIA_ValuePatternId)
        let pattern_obj = element.GetCurrentPattern(UIA_ValuePatternId)?;
        let value_pattern: IUIAutomationValuePattern = pattern_obj.cast()?;
        let bstr = BSTR::from(value);
        value_pattern.SetValue(&bstr)
    }
}
