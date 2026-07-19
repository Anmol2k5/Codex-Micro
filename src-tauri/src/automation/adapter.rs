use async_trait::async_trait;
use thiserror::Error;

use crate::core::models::{
    ActionResult, ActionTarget, CapabilitySet, CodexAction, ConnectionState, ThreadSummary,
};

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AutomationError {
    #[error("target application is not running")]
    AppNotRunning,
    #[error("Codex mode could not be detected")]
    CodexModeNotDetected,
    #[error("automation permission or integrity level blocked the action")]
    PermissionDenied,
    #[error("target UI element was not found: {0}")]
    ElementNotFound(String),
    #[error("automation action timed out: {0}")]
    TimedOut(String),
    #[error("automation result was ambiguous: {0}")]
    Ambiguous(String),
    #[error("action is unsupported: {0}")]
    Unsupported(String),
    #[error("automation failed: {0}")]
    Failed(String),
}

#[async_trait]
pub trait AutomationAdapter: Send + Sync {
    async fn connection_state(&self) -> Result<ConnectionState, AutomationError>;
    async fn capabilities(&self) -> Result<CapabilitySet, AutomationError>;
    async fn list_threads(&self) -> Result<Vec<ThreadSummary>, AutomationError>;
    async fn active_thread(&self) -> Result<Option<ThreadSummary>, AutomationError>;
    async fn reasoning_options(&self) -> Result<Vec<String>, AutomationError>;
    async fn execute(
        &self,
        action: CodexAction,
        target: ActionTarget,
    ) -> Result<ActionResult, AutomationError>;
}
