use std::time::Instant;

use async_trait::async_trait;

use crate::{
    automation::adapter::{AutomationAdapter, AutomationError},
    core::models::{
        ActionOutcome, ActionResult, ActionTarget, CapabilitySet, CodexAction, ConnectionState,
        ThreadSummary,
    },
};

use super::uia_worker::UiaWorker;
use super::selector_profile::SelectorProfile;
use super::TargetAppLocator;

pub struct WindowsAutomationAdapter {
    locator: TargetAppLocator,
    worker: UiaWorker,
    profile: SelectorProfile,
}

impl Default for WindowsAutomationAdapter {
    fn default() -> Self {
        let profile_str = include_str!("../../../resources/selectors/windows/default.json");
        let profile = SelectorProfile::parse(profile_str).expect("Failed to parse default selector profile");
        Self {
            locator: TargetAppLocator::default(),
            worker: UiaWorker::start(),
            profile,
        }
    }
}

impl WindowsAutomationAdapter {
    fn result(
        action: CodexAction,
        target: ActionTarget,
        outcome: ActionOutcome,
        code: &str,
        message: &str,
        started: Instant,
    ) -> ActionResult {
        ActionResult {
            action,
            target,
            outcome,
            user_message: message.into(),
            diagnostic_code: code.into(),
            elapsed_ms: started.elapsed().as_millis() as u64,
        }
    }
}

#[async_trait]
impl AutomationAdapter for WindowsAutomationAdapter {
    async fn connection_state(&self) -> Result<ConnectionState, AutomationError> {
        let Some(target) = self.locator.locate() else {
            return Ok(ConnectionState::NotRunning);
        };

        let (tx, rx) = tokio::sync::oneshot::channel();
        if self.worker.send(super::uia_worker::UiaCommand::GetConnectionState {
            target,
            profile: Some(self.profile.clone()),
            reply: tx,
        }).is_err() {
            return Ok(ConnectionState::Degraded);
        }

        match tokio::time::timeout(std::time::Duration::from_secs(2), rx).await {
            Ok(Ok(state)) => Ok(state),
            _ => Ok(ConnectionState::Degraded),
        }
    }

    async fn capabilities(&self) -> Result<CapabilitySet, AutomationError> {
        let Some(target) = self.locator.locate() else {
            return Ok(CapabilitySet::none());
        };

        let (tx, rx) = tokio::sync::oneshot::channel();
        if self.worker.send(super::uia_worker::UiaCommand::GetCapabilities {
            target,
            profile: self.profile.clone(),
            reply: tx,
        }).is_err() {
            return Ok(CapabilitySet::none());
        }

        match tokio::time::timeout(std::time::Duration::from_secs(2), rx).await {
            Ok(Ok(caps)) => Ok(caps),
            _ => Ok(CapabilitySet::focus_only()),
        }
    }

    async fn list_threads(&self) -> Result<Vec<ThreadSummary>, AutomationError> {
        let Some(target) = self.locator.locate() else {
            return Ok(Vec::new());
        };

        let (tx, rx) = tokio::sync::oneshot::channel();
        if self.worker.send(super::uia_worker::UiaCommand::GetThreads {
            target,
            profile: self.profile.clone(),
            reply: tx,
        }).is_err() {
            return Ok(Vec::new());
        }

        match tokio::time::timeout(std::time::Duration::from_secs(3), rx).await {
            Ok(Ok(Ok(threads))) => Ok(threads),
            _ => Ok(Vec::new()),
        }
    }

    async fn active_thread(&self) -> Result<Option<ThreadSummary>, AutomationError> {
        let threads = self.list_threads().await?;
        Ok(threads.into_iter().find(|t| t.is_active))
    }

    async fn reasoning_options(&self) -> Result<Vec<String>, AutomationError> {
        Ok(vec!["Low".to_string(), "Medium".to_string(), "High".to_string()])
    }

    async fn execute(
        &self,
        action: CodexAction,
        target: ActionTarget,
    ) -> Result<ActionResult, AutomationError> {
        let started = Instant::now();

        let Some(window) = self.locator.locate() else {
            return Ok(Self::result(
                action,
                target,
                ActionOutcome::AppNotRunning,
                "TARGET_APP_NOT_FOUND",
                "ChatGPT/Codex was not found. Open the desktop app and try again.",
                started,
            ));
        };

        if matches!(action, CodexAction::FocusApp) {
            if self.locator.focus(&window) {
                return Ok(Self::result(
                    action,
                    target,
                    ActionOutcome::Succeeded,
                    "WINDOW_FOCUSED",
                    "The detected ChatGPT/Codex window was brought to the foreground.",
                    started,
                ));
            } else {
                return Ok(Self::result(
                    action,
                    target,
                    ActionOutcome::PermissionDenied,
                    "TARGET_PRIVILEGE_MISMATCH",
                    "Windows refused to foreground the target. Ensure MicroDeck and ChatGPT run at matching privilege levels.",
                    started,
                ));
            }
        }

        // Delegate to the background worker thread
        let (tx, rx) = tokio::sync::oneshot::channel();
        if self.worker.send(super::uia_worker::UiaCommand::ExecuteAction {
            target: window,
            profile: self.profile.clone(),
            action: action.clone(),
            reply: tx,
        }).is_err() {
            return Ok(Self::result(
                action,
                target,
                ActionOutcome::Failed,
                "WORKER_COMMUNICATION_ERROR",
                "Failed to send command to the automation background thread.",
                started,
            ));
        }

        match tokio::time::timeout(std::time::Duration::from_secs(5), rx).await {
            Ok(Ok(res)) => Ok(res),
            _ => Ok(Self::result(
                action,
                target,
                ActionOutcome::TimedOut,
                "ACTION_CONFIRMATION_TIMEOUT",
                "The action was sent, but the confirmation response timed out.",
                started,
            )),
        }
    }
}
