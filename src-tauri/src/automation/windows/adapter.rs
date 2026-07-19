use std::time::Instant;

use async_trait::async_trait;

use crate::{
    automation::adapter::{AutomationAdapter, AutomationError},
    core::models::{
        ActionOutcome, ActionResult, ActionTarget, CapabilitySet, CodexAction, ConnectionState,
        ThreadSummary,
    },
};

use super::TargetAppLocator;

#[derive(Default)]
pub struct WindowsAutomationAdapter {
    locator: TargetAppLocator,
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

        if self.locator.is_foreground(&target) {
            Ok(ConnectionState::Connected)
        } else {
            Ok(ConnectionState::RunningNotFocused)
        }
    }

    async fn capabilities(&self) -> Result<CapabilitySet, AutomationError> {
        Ok(if self.locator.locate().is_some() {
            CapabilitySet::focus_only()
        } else {
            CapabilitySet::none()
        })
    }

    async fn list_threads(&self) -> Result<Vec<ThreadSummary>, AutomationError> {
        Ok(Vec::new())
    }

    async fn active_thread(&self) -> Result<Option<ThreadSummary>, AutomationError> {
        Ok(None)
    }

    async fn reasoning_options(&self) -> Result<Vec<String>, AutomationError> {
        Ok(Vec::new())
    }

    async fn execute(
        &self,
        action: CodexAction,
        target: ActionTarget,
    ) -> Result<ActionResult, AutomationError> {
        let started = Instant::now();

        if !matches!(action, CodexAction::FocusApp) {
            return Ok(Self::result(
                action,
                target,
                ActionOutcome::Unsupported,
                "UIA_SELECTOR_NOT_VERIFIED",
                "This Codex control is disabled until its Windows UI Automation selector is verified on the installed app version.",
                started,
            ));
        }

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

        if self.locator.focus(&window) {
            Ok(Self::result(
                action,
                target,
                ActionOutcome::Succeeded,
                "WINDOW_FOCUSED",
                "The detected ChatGPT/Codex window was brought to the foreground.",
                started,
            ))
        } else {
            Ok(Self::result(
                action,
                target,
                ActionOutcome::PermissionDenied,
                "TARGET_PRIVILEGE_MISMATCH",
                "Windows refused to foreground the target. Ensure MicroDeck and ChatGPT run at matching privilege levels.",
                started,
            ))
        }
    }
}
