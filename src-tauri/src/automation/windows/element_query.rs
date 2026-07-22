use crate::automation::windows::selector_profile::ElementSelectorCandidate;
use windows::{
    core::*,
    Win32::UI::Accessibility::*,
};

#[allow(dead_code)]
pub fn control_type_to_id(name: &str) -> Option<UIA_CONTROLTYPE_ID> {
    let clean = name.strip_prefix("ControlType.").unwrap_or(name);
    let id = match clean.to_lowercase().as_str() {
        "button" => 50000,
        "calendar" => 50001,
        "checkbox" => 50002,
        "combobox" => 50003,
        "edit" => 50004,
        "hyperlink" => 50005,
        "image" => 50006,
        "listitem" => 50007,
        "list" => 50008,
        "menu" => 50009,
        "menubar" => 50010,
        "menuitem" => 50011,
        "progressbar" => 50012,
        "radiobutton" => 50013,
        "scrollbar" => 50014,
        "slider" => 50015,
        "spinner" => 50016,
        "statusbar" => 50017,
        "tab" => 50018,
        "tabitem" => 50019,
        "text" => 50020,
        "toolbar" => 50021,
        "tooltip" => 50022,
        "treeview" => 50023,
        "treeitem" => 50024,
        "custom" => 50025,
        "group" => 50026,
        "document" => 50030,
        "pane" => 50033,
        _ => return None,
    };
    Some(UIA_CONTROLTYPE_ID(id))
}

#[allow(dead_code)]
pub fn pattern_name_to_id(name: &str) -> Option<UIA_PATTERN_ID> {
    let clean = name.strip_suffix("Identifiers.Pattern").unwrap_or(name);
    let id = match clean.to_lowercase().as_str() {
        "invokepattern" | "invoke" => 10000,
        "selectionpattern" | "selection" => 10001,
        "valuepattern" | "value" => 10002,
        "expandcollapsepattern" | "expandcollapse" => 10005,
        "selectionitempattern" | "selectionitem" => 10010,
        "textpattern" | "text" => 10014,
        "togglepattern" | "toggle" => 10015,
        "scrollitempattern" | "scrollitem" => 10017,
        "legacyiaccessiblepattern" | "legacyiaccessible" => 10018,
        _ => return None,
    };
    Some(UIA_PATTERN_ID(id))
}

#[allow(dead_code)]
fn score_element(element: &IUIAutomationElement, candidate: &ElementSelectorCandidate) -> Option<u32> {
    let mut score = 1; // Base score for a valid node

    // 1. Automation ID match
    if let Some(ref target_id) = candidate.automation_id {
        let current_id = unsafe { element.CurrentAutomationId().ok()?.to_string() };
        if current_id == *target_id {
            score += 100;
        } else {
            return None; // Hard mismatch
        }
    }

    // 2. Control Type match
    if let Some(ref target_type_str) = candidate.control_type {
        if let Some(target_type_id) = control_type_to_id(target_type_str) {
            let current_type_id = unsafe { element.CurrentControlType().ok()? };
            if current_type_id == target_type_id {
                score += 50;
            } else {
                return None; // Hard mismatch
            }
        }
    }

    // 3. Class Name match
    if let Some(ref target_class) = candidate.class_name {
        let current_class = unsafe { element.CurrentClassName().ok()?.to_string() };
        if current_class.contains(target_class) {
            score += 30;
        } else {
            return None; // Hard mismatch
        }
    }

    // 4. Name match
    if let Some(ref target_names) = candidate.names {
        let current_name = unsafe { element.CurrentName().ok()?.to_string() };
        let name_matches = target_names.iter().any(|name| current_name.eq_ignore_ascii_case(name));
        if name_matches {
            score += 50;
        } else {
            return None; // Hard mismatch
        }
    }

    // 5. Pattern match
    if let Some(ref target_patterns) = candidate.required_patterns {
        for pattern_str in target_patterns {
            if let Some(pattern_id) = pattern_name_to_id(pattern_str) {
                let has_pattern = unsafe {
                    // GetCurrentPattern returns Ok(IUnknown) if supported
                    element.GetCurrentPattern(pattern_id).is_ok()
                };
                if has_pattern {
                    score += 10;
                } else {
                    return None; // Hard mismatch
                }
            }
        }
    }

    Some(score)
}

#[allow(dead_code)]
fn resolve_ancestor_and_descendant_hints(
    walker: &IUIAutomationTreeWalker,
    element: &IUIAutomationElement,
    candidate: &ElementSelectorCandidate,
    score: &mut u32,
) {
    // Ancestor hints validation
    if let Some(ref ancestors) = candidate.ancestor_hints {
        let mut current_ancestor = unsafe { walker.GetParentElement(element).ok() };
        let mut matched_ancestors = 0;
        let mut depth = 0;

        while let Some(ancestor) = current_ancestor {
            if depth > 10 {
                break; // Limit traversal depth
            }
            let class_name = unsafe { ancestor.CurrentClassName().map(|b| b.to_string()).unwrap_or_default() };
            let name = unsafe { ancestor.CurrentName().map(|b| b.to_string()).unwrap_or_default() };

            for hint in ancestors {
                if class_name.contains(hint) || name.contains(hint) {
                    matched_ancestors += 1;
                }
            }
            current_ancestor = unsafe { walker.GetParentElement(&ancestor).ok() };
            depth += 1;
        }
        *score += matched_ancestors * 20;
    }
}

#[allow(dead_code)]
pub fn find_matching_elements(
    walker: &IUIAutomationTreeWalker,
    element: &IUIAutomationElement,
    candidate: &ElementSelectorCandidate,
    depth: usize,
    max_depth: usize,
    matches: &mut Vec<(IUIAutomationElement, u32)>,
) -> Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    if let Some(mut score) = score_element(element, candidate) {
        resolve_ancestor_and_descendant_hints(walker, element, candidate, &mut score);
        matches.push((element.clone(), score));
    }

    unsafe {
        if let Ok(child) = walker.GetFirstChildElement(element) {
            let mut current = child;
            loop {
                let _ = find_matching_elements(walker, &current, candidate, depth + 1, max_depth, matches);
                current = match walker.GetNextSiblingElement(&current) {
                    Ok(sib) => sib,
                    Err(_) => break,
                };
            }
        }
    }

    Ok(())
}

/// Resolves the single best matching UI Automation element for a set of selector candidates.
#[allow(dead_code)]
pub fn resolve_element(
    walker: &IUIAutomationTreeWalker,
    root: &IUIAutomationElement,
    candidates: &[ElementSelectorCandidate],
) -> Result<IUIAutomationElement> {
    if candidates.is_empty() {
        return Err(Error::new(HRESULT(-2147467259), "No candidates provided")); // E_FAIL
    }

    let mut all_matches = Vec::new();

    for candidate in candidates {
        let mut matches = Vec::new();
        let _ = find_matching_elements(walker, root, candidate, 0, 25, &mut matches);
        all_matches.extend(matches);
    }

    if all_matches.is_empty() {
        return Err(Error::new(HRESULT(-2147467259), "No matching elements found"));
    }

    // Sort descending by score
    all_matches.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(all_matches[0].0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_control_type_names_correctly() {
        assert_eq!(control_type_to_id("ControlType.Button"), Some(UIA_CONTROLTYPE_ID(50000)));
        assert_eq!(control_type_to_id("button"), Some(UIA_CONTROLTYPE_ID(50000)));
        assert_eq!(control_type_to_id("ControlType.Edit"), Some(UIA_CONTROLTYPE_ID(50004)));
        assert_eq!(control_type_to_id("document"), Some(UIA_CONTROLTYPE_ID(50030)));
        assert_eq!(control_type_to_id("non-existent"), None);
    }

    #[test]
    fn maps_pattern_names_correctly() {
        assert_eq!(pattern_name_to_id("InvokePattern"), Some(UIA_PATTERN_ID(10000)));
        assert_eq!(pattern_name_to_id("Invoke"), Some(UIA_PATTERN_ID(10000)));
        assert_eq!(pattern_name_to_id("ValuePatternIdentifiers.Pattern"), Some(UIA_PATTERN_ID(10002)));
        assert_eq!(pattern_name_to_id("non-existent"), None);
    }
}
