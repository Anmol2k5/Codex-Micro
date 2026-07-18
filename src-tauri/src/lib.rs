mod app_state;
mod automation;
mod commands;
mod core;

use std::sync::Arc;

use app_state::AppState;
use automation::{adapter::AutomationAdapter, mock::MockAutomationAdapter};
use commands::{
    execute_action, get_active_thread, get_capabilities, get_connection_state,
    get_reasoning_options, list_threads,
};

fn create_automation_adapter() -> Arc<dyn AutomationAdapter> {
    let force_mock = std::env::var("MICRODECK_USE_MOCK")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if force_mock {
        return Arc::new(MockAutomationAdapter::default());
    }

    #[cfg(windows)]
    {
        return Arc::new(automation::windows::WindowsAutomationAdapter::default());
    }

    #[cfg(not(windows))]
    {
        Arc::new(MockAutomationAdapter::default())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let automation = create_automation_adapter();

    tauri::Builder::default()
        .manage(AppState::new(automation))
        .invoke_handler(tauri::generate_handler![
            get_connection_state,
            get_capabilities,
            list_threads,
            get_active_thread,
            get_reasoning_options,
            execute_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running MicroDeck");
}
