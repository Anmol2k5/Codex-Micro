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
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;

fn create_automation_adapter() -> Arc<dyn AutomationAdapter> {
    let force_mock = std::env::var("MICRODECK_USE_MOCK")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if force_mock {
        return Arc::new(MockAutomationAdapter::default());
    }

    #[cfg(windows)]
    {
        Arc::new(automation::windows::WindowsAutomationAdapter::default())
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
        .setup(|app| {
            let show_dashboard = MenuItem::with_id(app, "show_dashboard", "Show Dashboard", true, None::<&str>)?;
            let show_controller = MenuItem::with_id(app, "show_controller", "Show Controller", true, None::<&str>)?;
            let exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_dashboard, &show_controller, &exit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show_dashboard" => {
                            if let Some(w) = app.get_webview_window("main") {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                        "show_controller" => {
                            if let Some(w) = app.get_webview_window("controller") {
                                let _ = w.show();
                                let _ = w.set_focus();
                            }
                        }
                        "exit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running MicroDeck");
}
