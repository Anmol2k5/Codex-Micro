use serde::{Deserialize, Serialize};

pub type ThreadId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionState {
    NotRunning,
    RunningNotFocused,
    Connected,
    CodexModeNotDetected,
    PermissionRequired,
    Degraded,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ThreadStatus {
    Working,
    Thinking,
    WaitingForUser,
    WaitingForApproval,
    Completed,
    Failed,
    Idle,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ThreadSummary {
    pub id: ThreadId,
    pub title: String,
    pub project: Option<String>,
    pub status: ThreadStatus,
    pub is_active: bool,
    pub updated_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum CodexAction {
    FocusApp,
    NewThread,
    ReviewChanges,
    Approve,
    Reject,
    DiscardChanges,
    SubmitPrompt { text: String },
    StartSystemDictation,
    SelectThread { thread_id: ThreadId },
    SetReasoningLevel { value: String },
    OpenShortcutHelp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum ActionTarget {
    ActiveThread,
    SelectedThread { thread_id: ThreadId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActionOutcome {
    Succeeded,
    Unsupported,
    TargetNotFound,
    PermissionDenied,
    TimedOut,
    Ambiguous,
    AppNotRunning,
    CodexModeNotDetected,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActionResult {
    pub action: CodexAction,
    pub target: ActionTarget,
    pub outcome: ActionOutcome,
    pub user_message: String,
    pub diagnostic_code: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitySet {
    pub can_focus_app: bool,
    pub can_list_threads: bool,
    pub can_select_thread: bool,
    pub can_create_thread: bool,
    pub can_review_changes: bool,
    pub can_approve: bool,
    pub can_reject: bool,
    pub can_discard_changes: bool,
    pub can_submit_prompt: bool,
    pub can_start_system_dictation: bool,
    pub can_read_reasoning_options: bool,
    pub can_set_reasoning_level: bool,
}

impl CapabilitySet {
    pub fn none() -> Self {
        Self {
            can_focus_app: false,
            can_list_threads: false,
            can_select_thread: false,
            can_create_thread: false,
            can_review_changes: false,
            can_approve: false,
            can_reject: false,
            can_discard_changes: false,
            can_submit_prompt: false,
            can_start_system_dictation: false,
            can_read_reasoning_options: false,
            can_set_reasoning_level: false,
        }
    }

    pub fn focus_only() -> Self {
        let mut capabilities = Self::none();
        capabilities.can_focus_app = true;
        capabilities
    }

    pub fn mock_full() -> Self {
        Self {
            can_focus_app: true,
            can_list_threads: true,
            can_select_thread: true,
            can_create_thread: true,
            can_review_changes: true,
            can_approve: true,
            can_reject: true,
            can_discard_changes: false,
            can_submit_prompt: true,
            can_start_system_dictation: false,
            can_read_reasoning_options: true,
            can_set_reasoning_level: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_only_does_not_claim_unverified_codex_controls() {
        let capabilities = CapabilitySet::focus_only();
        assert!(capabilities.can_focus_app);
        assert!(!capabilities.can_list_threads);
        assert!(!capabilities.can_approve);
        assert!(!capabilities.can_set_reasoning_level);
    }
}
