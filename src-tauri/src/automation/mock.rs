use std::time::Instant;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    automation::adapter::{AutomationAdapter, AutomationError},
    core::models::{
        ActionOutcome, ActionResult, ActionTarget, CapabilitySet, CodexAction, ConnectionState,
        ThreadStatus, ThreadSummary,
    },
};

pub struct MockAutomationAdapter {
    threads: RwLock<Vec<ThreadSummary>>,
}

impl Default for MockAutomationAdapter {
    fn default() -> Self {
        Self {
            threads: RwLock::new(vec![
                ThreadSummary {
                    id: "thread-1".into(),
                    title: "Implement auth flow".into(),
                    project: Some("MicroDeck".into()),
                    status: ThreadStatus::Working,
                    is_active: true,
                    updated_at_ms: None,
                },
                ThreadSummary {
                    id: "thread-2".into(),
                    title: "Review Windows adapter".into(),
                    project: Some("MicroDeck".into()),
                    status: ThreadStatus::WaitingForApproval,
                    is_active: false,
                    updated_at_ms: None,
                },
            ]),
        }
    }
}

impl MockAutomationAdapter {
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
impl AutomationAdapter for MockAutomationAdapter {
    async fn connection_state(&self) -> Result<ConnectionState, AutomationError> {
        Ok(ConnectionState::Connected)
    }

    async fn capabilities(&self) -> Result<CapabilitySet, AutomationError> {
        Ok(CapabilitySet::mock_full())
    }

    async fn list_threads(&self) -> Result<Vec<ThreadSummary>, AutomationError> {
        Ok(self.threads.read().await.clone())
    }

    async fn active_thread(&self) -> Result<Option<ThreadSummary>, AutomationError> {
        Ok(self
            .threads
            .read()
            .await
            .iter()
            .find(|thread| thread.is_active)
            .cloned())
    }

    async fn reasoning_options(&self) -> Result<Vec<String>, AutomationError> {
        Ok(vec!["Low".into(), "Medium".into(), "High".into()])
    }

    async fn execute(
        &self,
        action: CodexAction,
        target: ActionTarget,
    ) -> Result<ActionResult, AutomationError> {
        let started = Instant::now();

        if let ActionTarget::SelectedThread { thread_id } = &target {
            let exists = self
                .threads
                .read()
                .await
                .iter()
                .any(|thread| &thread.id == thread_id);
            if !exists {
                return Ok(Self::result(
                    action,
                    target,
                    ActionOutcome::TargetNotFound,
                    "MOCK_TARGET_NOT_FOUND",
                    "The selected mock thread no longer exists.",
                    started,
                ));
            }
        }

        match &action {
            CodexAction::SelectThread { thread_id } => {
                let mut threads = self.threads.write().await;
                let mut found = false;
                for thread in threads.iter_mut() {
                    thread.is_active = thread.id == *thread_id;
                    found |= thread.is_active;
                }
                if !found {
                    return Ok(Self::result(
                        action,
                        target,
                        ActionOutcome::TargetNotFound,
                        "MOCK_THREAD_NOT_FOUND",
                        "The requested mock thread was not found.",
                        started,
                    ));
                }
            }
            CodexAction::NewThread => {
                let mut threads = self.threads.write().await;
                for thread in threads.iter_mut() {
                    thread.is_active = false;
                }
                let id = format!("thread-{}", threads.len() + 1);
                threads.push(ThreadSummary {
                    id,
                    title: "New Codex task".into(),
                    project: Some("MicroDeck".into()),
                    status: ThreadStatus::Idle,
                    is_active: true,
                    updated_at_ms: None,
                });
            }
            CodexAction::StartSystemDictation | CodexAction::DiscardChanges => {
                return Ok(Self::result(
                    action,
                    target,
                    ActionOutcome::Unsupported,
                    "MOCK_ACTION_UNSUPPORTED",
                    "This action is intentionally disabled in the mock adapter.",
                    started,
                ));
            }
            _ => {}
        }

        Ok(Self::result(
            action,
            target,
            ActionOutcome::Succeeded,
            "MOCK_ACTION_SUCCEEDED",
            "Mock action completed and observable state was updated when applicable.",
            started,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn selected_missing_thread_returns_target_not_found() {
        let adapter = MockAutomationAdapter::default();
        let result = adapter
            .execute(
                CodexAction::Approve,
                ActionTarget::SelectedThread {
                    thread_id: "missing".into(),
                },
            )
            .await
            .expect("mock execution should not error");

        assert_eq!(result.outcome, ActionOutcome::TargetNotFound);
    }

    #[tokio::test]
    async fn selecting_thread_changes_active_thread() {
        let adapter = MockAutomationAdapter::default();
        adapter
            .execute(
                CodexAction::SelectThread {
                    thread_id: "thread-2".into(),
                },
                ActionTarget::ActiveThread,
            )
            .await
            .expect("mock execution should not error");

        assert_eq!(
            adapter.active_thread().await.unwrap().unwrap().id,
            "thread-2"
        );
    }
}
