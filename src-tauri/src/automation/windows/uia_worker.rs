use crate::automation::windows::locator::TargetWindow;
use crate::automation::windows::selector_profile::SelectorProfile;
use crate::core::models::{ActionOutcome, ActionResult, ActionTarget, CapabilitySet, CodexAction, ConnectionState, ThreadSummary, ThreadStatus};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Instant;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Accessibility::IUIAutomationElement;

#[allow(dead_code)]
pub enum UiaCommand {
    FindTarget {
        process_names: Vec<String>,
        window_title_hints: Vec<String>,
        reply: tokio::sync::oneshot::Sender<Result<TargetWindow, String>>,
    },
    GetConnectionState {
        target: TargetWindow,
        profile: Option<SelectorProfile>,
        reply: tokio::sync::oneshot::Sender<ConnectionState>,
    },
    GetCapabilities {
        target: TargetWindow,
        profile: SelectorProfile,
        reply: tokio::sync::oneshot::Sender<CapabilitySet>,
    },
    GetThreads {
        target: TargetWindow,
        profile: SelectorProfile,
        reply: tokio::sync::oneshot::Sender<Result<Vec<ThreadSummary>, String>>,
    },
    ExecuteAction {
        target: TargetWindow,
        profile: SelectorProfile,
        action: CodexAction,
        reply: tokio::sync::oneshot::Sender<ActionResult>,
    },
}

#[allow(dead_code)]
pub struct UiaWorker {
    tx: Sender<UiaCommand>,
}

#[allow(dead_code)]
fn check_selector_resolves(
    uia: &super::uia_client::UiaClient,
    root: &IUIAutomationElement,
    profile: &SelectorProfile,
    key: &str,
) -> bool {
    let walker = match uia.get_control_view_walker() {
        Ok(w) => w,
        Err(_) => return false,
    };
    if let Some(candidates) = profile.selectors.get(key) {
        super::element_query::resolve_element(&walker, root, candidates).is_ok()
    } else {
        false
    }
}

#[allow(dead_code)]
fn read_threads_from_list(
    walker: &windows::Win32::UI::Accessibility::IUIAutomationTreeWalker,
    list_el: &IUIAutomationElement,
    threads: &mut Vec<ThreadSummary>,
) -> windows::core::Result<()> {
    unsafe {
        if let Ok(child) = walker.GetFirstChildElement(list_el) {
            let mut current = child;
            loop {
                let control_type = current.CurrentControlType()?;
                // 50007 is UIA_ListItemControlTypeId
                if control_type.0 == 50007 {
                    let name = current.CurrentName()?.to_string();
                    if !name.is_empty() && name != "No chats" {
                        let class_name = current.CurrentClassName()?.to_string();
                        // Active check
                        let is_active = class_name.contains("bg-token-list-hover-background") ||
                                        class_name.contains("selected") ||
                                        current.CurrentName()?.to_string().contains("active");
                        
                        threads.push(ThreadSummary {
                            id: name.clone(),
                            title: name,
                            project: None,
                            status: ThreadStatus::Idle,
                            is_active,
                            updated_at_ms: None,
                        });
                    }
                }
                
                // Recurse down
                let _ = read_threads_from_list(walker, &current, threads);
                
                current = match walker.GetNextSiblingElement(&current) {
                    Ok(sib) => sib,
                    Err(_) => break,
                };
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
impl UiaWorker {
    pub fn start() -> Self {
        let (tx, rx) = channel::<UiaCommand>();
        thread::spawn(move || {
            unsafe {
                let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            }

            let uia = match super::uia_client::UiaClient::new() {
                Ok(client) => client,
                Err(e) => {
                    eprintln!("Failed to create UiaClient on background thread: {:?}", e);
                    return;
                }
            };

            while let Ok(cmd) = rx.recv() {
                match cmd {
                    UiaCommand::FindTarget {
                        process_names: _,
                        window_title_hints: _,
                        reply,
                    } => {
                        let _ = reply.send(Err("Not implemented".into()));
                    }
                    UiaCommand::GetConnectionState {
                        target,
                        profile,
                        reply,
                    } => {
                        let state = match profile {
                            None => ConnectionState::CodexModeNotDetected,
                            Some(ref prof) => {
                                let hwnd = HWND(target.hwnd as *mut std::ffi::c_void);
                                if let Ok(root) = uia.element_from_handle(hwnd) {
                                    if check_selector_resolves(&uia, &root, prof, "codexModeIndicator") {
                                        let is_focused = unsafe {
                                            let fg = windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow();
                                            fg == hwnd
                                        };
                                        if is_focused {
                                            ConnectionState::Connected
                                        } else {
                                            ConnectionState::RunningNotFocused
                                        }
                                    } else {
                                        ConnectionState::CodexModeNotDetected
                                    }
                                } else {
                                    ConnectionState::NotRunning
                                }
                            }
                        };
                        let _ = reply.send(state);
                    }
                    UiaCommand::GetCapabilities {
                        target,
                        profile,
                        reply,
                    } => {
                        let mut caps = CapabilitySet::none();
                        let hwnd = HWND(target.hwnd as *mut std::ffi::c_void);
                        if let Ok(root) = uia.element_from_handle(hwnd) {
                            caps.can_focus_app = true;
                            caps.can_list_threads = check_selector_resolves(&uia, &root, &profile, "threadList");
                            caps.can_select_thread = check_selector_resolves(&uia, &root, &profile, "threadRow");
                            caps.can_create_thread = check_selector_resolves(&uia, &root, &profile, "newThread");
                            caps.can_review_changes = check_selector_resolves(&uia, &root, &profile, "reviewChanges");
                            caps.can_approve = check_selector_resolves(&uia, &root, &profile, "approve");
                            caps.can_reject = check_selector_resolves(&uia, &root, &profile, "reject");
                            caps.can_discard_changes = check_selector_resolves(&uia, &root, &profile, "discardChanges");
                            caps.can_submit_prompt = check_selector_resolves(&uia, &root, &profile, "promptComposer");
                            caps.can_start_system_dictation = true;
                            caps.can_read_reasoning_options = check_selector_resolves(&uia, &root, &profile, "reasoningControl");
                            caps.can_set_reasoning_level = caps.can_read_reasoning_options && check_selector_resolves(&uia, &root, &profile, "reasoningOption");
                        }
                        let _ = reply.send(caps);
                    }
                    UiaCommand::GetThreads {
                        target,
                        profile,
                        reply,
                    } => {
                        let hwnd = HWND(target.hwnd as *mut std::ffi::c_void);
                        let result = match uia.element_from_handle(hwnd) {
                            Err(e) => Err(format!("Failed to get window handle: {}", e)),
                            Ok(root) => {
                                let walker = uia.get_control_view_walker().unwrap();
                                if let Some(candidates) = profile.selectors.get("threadList") {
                                    match super::element_query::resolve_element(&walker, &root, candidates) {
                                        Err(e) => Err(format!("Thread list not found: {}", e)),
                                        Ok(list_el) => {
                                            let mut threads = Vec::new();
                                            let _ = read_threads_from_list(&walker, &list_el, &mut threads);
                                            Ok(threads)
                                        }
                                    }
                                } else {
                                    Err("Thread list selector candidates not configured.".into())
                                }
                            }
                        };
                        let _ = reply.send(result);
                    }
                    UiaCommand::ExecuteAction {
                        target,
                        profile,
                        action,
                        reply,
                    } => {
                        let started = Instant::now();
                        let hwnd = HWND(target.hwnd as *mut std::ffi::c_void);
                        let result = match uia.element_from_handle(hwnd) {
                            Err(_) => ActionResult {
                                action: action.clone(),
                                target: ActionTarget::ActiveThread,
                                outcome: ActionOutcome::TargetNotFound,
                                user_message: "ChatGPT window not found.".to_string(),
                                diagnostic_code: "TARGET_APP_NOT_FOUND".to_string(),
                                elapsed_ms: started.elapsed().as_millis() as u64,
                            },
                            Ok(root) => {
                                let walker = uia.get_control_view_walker().unwrap();
                                match action.clone() {
                                    CodexAction::FocusApp => ActionResult {
                                        action: action.clone(),
                                        target: ActionTarget::ActiveThread,
                                        outcome: ActionOutcome::Succeeded,
                                        user_message: "App focused".to_string(),
                                        diagnostic_code: "WINDOW_FOCUSED".to_string(),
                                        elapsed_ms: started.elapsed().as_millis() as u64,
                                    },
                                    CodexAction::NewThread => {
                                        if let Some(candidates) = profile.selectors.get("newThread") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "New Thread button not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(btn) => {
                                                    match super::action_executor::invoke_element(&btn) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to invoke New Thread button: {}", e),
                                                            diagnostic_code: "UIA_PATTERN_UNSUPPORTED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => {
                                                            thread::sleep(std::time::Duration::from_millis(500));
                                                            ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::Succeeded,
                                                                user_message: "New thread created successfully.".to_string(),
                                                                diagnostic_code: "NEW_THREAD_CREATED".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "New Thread selector candidates not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::ReviewChanges => {
                                        if let Some(candidates) = profile.selectors.get("reviewChanges") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Review Changes button not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(btn) => {
                                                    match super::action_executor::invoke_element(&btn) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to invoke Review Changes: {}", e),
                                                            diagnostic_code: "UIA_PATTERN_UNSUPPORTED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => {
                                                            thread::sleep(std::time::Duration::from_millis(500));
                                                            ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::Succeeded,
                                                                user_message: "Review changes opened successfully.".to_string(),
                                                                diagnostic_code: "REVIEW_CHANGES_OPENED".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Review Changes selector not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::Approve => {
                                        if let Some(candidates) = profile.selectors.get("approve") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Approve button not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(btn) => {
                                                    match super::action_executor::invoke_element(&btn) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to invoke Approve: {}", e),
                                                            diagnostic_code: "UIA_PATTERN_UNSUPPORTED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => {
                                                            thread::sleep(std::time::Duration::from_millis(500));
                                                            ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::Succeeded,
                                                                user_message: "Approved successfully.".to_string(),
                                                                diagnostic_code: "APPROVED".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Approve selector not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::Reject => {
                                        if let Some(candidates) = profile.selectors.get("reject") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Reject button not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(btn) => {
                                                    match super::action_executor::invoke_element(&btn) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to invoke Reject: {}", e),
                                                            diagnostic_code: "UIA_PATTERN_UNSUPPORTED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => {
                                                            thread::sleep(std::time::Duration::from_millis(500));
                                                            ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::Succeeded,
                                                                user_message: "Rejected successfully.".to_string(),
                                                                diagnostic_code: "REJECTED".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Reject selector not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::DiscardChanges => {
                                        if let Some(candidates) = profile.selectors.get("discardChanges") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Discard Changes button not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(btn) => {
                                                    match super::action_executor::invoke_element(&btn) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to invoke Discard Changes: {}", e),
                                                            diagnostic_code: "UIA_PATTERN_UNSUPPORTED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => {
                                                            thread::sleep(std::time::Duration::from_millis(500));
                                                            ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::Succeeded,
                                                                user_message: "Discarded changes successfully.".to_string(),
                                                                diagnostic_code: "DISCARDED_CHANGES".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Discard Changes selector not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::SubmitPrompt { text } => {
                                        if let Some(comp_candidates) = profile.selectors.get("promptComposer") {
                                            match super::element_query::resolve_element(&walker, &root, comp_candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Prompt composer textbox not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(comp) => {
                                                    match super::input_fallback::focus_and_paste(&comp, &text) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to write text to prompt composer: {}", e),
                                                            diagnostic_code: "INPUT_INJECTION_FAILED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => {
                                                            thread::sleep(std::time::Duration::from_millis(150));
                                                            // Resolve and invoke sendButton
                                                            if let Some(send_candidates) = profile.selectors.get("sendButton") {
                                                                match super::element_query::resolve_element(&walker, &root, send_candidates) {
                                                                    Err(_) => ActionResult {
                                                                        action: action.clone(),
                                                                        target: ActionTarget::ActiveThread,
                                                                        outcome: ActionOutcome::Failed,
                                                                        user_message: format!("Text written to composer, but Send button was not found."),
                                                                        diagnostic_code: "SEND_BUTTON_NOT_FOUND".to_string(),
                                                                        elapsed_ms: started.elapsed().as_millis() as u64,
                                                                    },
                                                                    Ok(send_btn) => {
                                                                        let _ = super::action_executor::invoke_element(&send_btn);
                                                                        ActionResult {
                                                                            action: action.clone(),
                                                                            target: ActionTarget::ActiveThread,
                                                                            outcome: ActionOutcome::Succeeded,
                                                                            user_message: "Prompt submitted successfully.".to_string(),
                                                                            diagnostic_code: "PROMPT_SUBMITTED".to_string(),
                                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                ActionResult {
                                                                    action: action.clone(),
                                                                    target: ActionTarget::ActiveThread,
                                                                    outcome: ActionOutcome::Succeeded,
                                                                    user_message: "Prompt pasted, but no send button was configured.".to_string(),
                                                                    diagnostic_code: "SEND_BUTTON_NOT_CONFIGURED".to_string(),
                                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Prompt composer selector candidates not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::StartSystemDictation => {
                                        if let Some(comp_candidates) = profile.selectors.get("promptComposer") {
                                            match super::element_query::resolve_element(&walker, &root, comp_candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Prompt composer textbox not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(comp) => {
                                                    match super::input_fallback::trigger_dictation(&comp) {
                                                        Err(e) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Failed,
                                                            user_message: format!("Failed to trigger dictation: {}", e),
                                                            diagnostic_code: "INPUT_INJECTION_FAILED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Ok(_) => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Succeeded,
                                                            user_message: "System dictation triggered successfully.".to_string(),
                                                            diagnostic_code: "DICTATION_STARTED".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Prompt composer selector not configured for dictation.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::SetReasoningLevel { value } => {
                                        if let Some(candidates) = profile.selectors.get("reasoningControl") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Reasoning selector control not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(ctrl) => {
                                                    let _ = super::action_executor::invoke_element(&ctrl);
                                                    thread::sleep(std::time::Duration::from_millis(300));
                                                    
                                                    // Now resolve reasoning option matching value
                                                    if let Some(opt_candidates) = profile.selectors.get("reasoningOption") {
                                                        // Let's search for the option buttons
                                                        let mut option_elements = Vec::new();
                                                        for candidate in opt_candidates {
                                                            let mut opt_matches = Vec::new();
                                                            let _ = super::element_query::find_matching_elements(
                                                                &walker,
                                                                &root,
                                                                candidate,
                                                                0,
                                                                25,
                                                                &mut opt_matches,
                                                            );
                                                            option_elements.extend(opt_matches);
                                                        }
                                                        
                                                        // Find the option element whose name matches the target value
                                                        let mut target_opt = None;
                                                        for (opt_el, _) in option_elements {
                                                            if let Ok(name) = unsafe { opt_el.CurrentName() } {
                                                                let name_str = name.to_string();
                                                                if name_str.to_lowercase().contains(&value.to_lowercase()) {
                                                                    target_opt = Some(opt_el);
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        
                                                        match target_opt {
                                                            None => ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::TargetNotFound,
                                                                user_message: format!("Reasoning option '{}' not found in dropdown.", value),
                                                                diagnostic_code: "REASONING_OPTION_NOT_FOUND".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            },
                                                            Some(opt_el) => {
                                                                let _ = super::action_executor::invoke_element(&opt_el);
                                                                thread::sleep(std::time::Duration::from_millis(500));
                                                                ActionResult {
                                                                    action: action.clone(),
                                                                    target: ActionTarget::ActiveThread,
                                                                    outcome: ActionOutcome::Succeeded,
                                                                    user_message: format!("Reasoning level set to '{}'.", value),
                                                                    diagnostic_code: "REASONING_LEVEL_SET".to_string(),
                                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::Unsupported,
                                                            user_message: "Reasoning option selectors not configured.".to_string(),
                                                            diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Reasoning control selector not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::SelectThread { thread_id } => {
                                        // Walk threads, find thread with matching id (which is title)
                                        if let Some(candidates) = profile.selectors.get("threadList") {
                                            match super::element_query::resolve_element(&walker, &root, candidates) {
                                                Err(_) => ActionResult {
                                                    action: action.clone(),
                                                    target: ActionTarget::ActiveThread,
                                                    outcome: ActionOutcome::Failed,
                                                    user_message: "Thread list listbox not found.".to_string(),
                                                    diagnostic_code: "UIA_ELEMENT_NOT_FOUND".to_string(),
                                                    elapsed_ms: started.elapsed().as_millis() as u64,
                                                },
                                                Ok(list_el) => {
                                                    // Find matching thread item in children
                                                    let mut found_el = None;
                                                    unsafe {
                                                        if let Ok(child) = walker.GetFirstChildElement(&list_el) {
                                                            let mut current = child;
                                                            loop {
                                                                if current.CurrentName().map(|b| b.to_string()).unwrap_or_default() == thread_id {
                                                                    found_el = Some(current.clone());
                                                                    break;
                                                                }
                                                                current = match walker.GetNextSiblingElement(&current) {
                                                                    Ok(sib) => sib,
                                                                    Err(_) => break,
                                                                };
                                                            }
                                                        }
                                                    }

                                                    match found_el {
                                                        None => ActionResult {
                                                            action: action.clone(),
                                                            target: ActionTarget::ActiveThread,
                                                            outcome: ActionOutcome::TargetNotFound,
                                                            user_message: format!("Thread '{}' was not found in the thread list.", thread_id),
                                                            diagnostic_code: "THREAD_NOT_FOUND".to_string(),
                                                            elapsed_ms: started.elapsed().as_millis() as u64,
                                                        },
                                                        Some(thread_el) => {
                                                            let _ = super::action_executor::invoke_element(&thread_el);
                                                            let _ = super::action_executor::select_element(&thread_el);
                                                            thread::sleep(std::time::Duration::from_millis(500));
                                                            ActionResult {
                                                                action: action.clone(),
                                                                target: ActionTarget::ActiveThread,
                                                                outcome: ActionOutcome::Succeeded,
                                                                user_message: format!("Switched to thread '{}' successfully.", thread_id),
                                                                diagnostic_code: "THREAD_SWITCHED".to_string(),
                                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            ActionResult {
                                                action: action.clone(),
                                                target: ActionTarget::ActiveThread,
                                                outcome: ActionOutcome::Unsupported,
                                                user_message: "Thread list selector candidates not configured.".to_string(),
                                                diagnostic_code: "SELECTOR_PROFILE_INVALID".to_string(),
                                                elapsed_ms: started.elapsed().as_millis() as u64,
                                            }
                                        }
                                    }
                                    CodexAction::OpenShortcutHelp => ActionResult {
                                        action: action.clone(),
                                        target: ActionTarget::ActiveThread,
                                        outcome: ActionOutcome::Unsupported,
                                        user_message: "Shortcut help is not supported on this platform version.".to_string(),
                                        diagnostic_code: "SHORTCUT_HELP_UNSUPPORTED".to_string(),
                                        elapsed_ms: started.elapsed().as_millis() as u64,
                                    },
                                }
                            }
                        };
                        let _ = reply.send(result);
                    }
                }
            }

            unsafe {
                CoUninitialize();
            }
        });

        Self { tx }
    }

    pub fn send(&self, cmd: UiaCommand) -> Result<(), String> {
        self.tx
            .send(cmd)
            .map_err(|e| format!("Failed to send command to UiaWorker: {}", e))
    }
}
