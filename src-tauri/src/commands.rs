use serde::Deserialize;
use tauri::State;

use crate::{
    app_state::AppState,
    core::models::{
        ActionResult, ActionTarget, CapabilitySet, CodexAction, ConnectionState, ThreadSummary,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteActionRequest {
    pub action: CodexAction,
    pub target: ActionTarget,
}

#[tauri::command]
pub async fn get_connection_state(state: State<'_, AppState>) -> Result<ConnectionState, String> {
    state
        .automation
        .connection_state()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_capabilities(state: State<'_, AppState>) -> Result<CapabilitySet, String> {
    state
        .automation
        .capabilities()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_threads(state: State<'_, AppState>) -> Result<Vec<ThreadSummary>, String> {
    state
        .automation
        .list_threads()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_active_thread(
    state: State<'_, AppState>,
) -> Result<Option<ThreadSummary>, String> {
    state
        .automation
        .active_thread()
        .await
        .map_err(|error| error.to_string())
}


#[tauri::command]
pub async fn get_reasoning_options(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    state
        .automation
        .reasoning_options()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn execute_action(
    state: State<'_, AppState>,
    request: ExecuteActionRequest,
) -> Result<ActionResult, String> {
    state
        .automation
        .execute(request.action, request.target)
        .await
        .map_err(|error| error.to_string())
}
