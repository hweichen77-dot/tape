use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_window_state::StateFlags;

fn build_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let app_menu = Submenu::with_items(
        app,
        "Tape",
        true,
        &[
            &PredefinedMenuItem::about(app, None, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::services(app, None)?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )?;

    let run_menu = Submenu::with_items(
        app,
        "Run",
        true,
        &[
            &MenuItem::with_id(app, "new", "New Run", true, Some("CmdOrCtrl+N"))?,
            &MenuItem::with_id(app, "resume", "Resume Saved Run", true, Some("CmdOrCtrl+R"))?,
            &MenuItem::with_id(app, "mods", "Modifiers…", true, Some("CmdOrCtrl+,"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "playpause", "Play / Pause", true, Some("CmdOrCtrl+P"))?,
            &MenuItem::with_id(app, "flatten", "Flatten All", true, Some("CmdOrCtrl+Shift+F"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "share", "Copy Score Card", true, Some("CmdOrCtrl+Shift+C"))?,
        ],
    )?;

    let view_menu = Submenu::with_items(
        app,
        "View",
        true,
        &[
            &MenuItem::with_id(app, "vwap", "Toggle VWAP", true, Some("CmdOrCtrl+Shift+V"))?,
            &MenuItem::with_id(app, "sound", "Toggle Sound", true, Some("CmdOrCtrl+Shift+S"))?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::fullscreen(app, None)?,
        ],
    )?;

    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;

    let window_menu = Submenu::with_items(
        app,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None)?,
            &PredefinedMenuItem::maximize(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::close_window(app, None)?,
        ],
    )?;

    Menu::with_items(
        app,
        &[&app_menu, &run_menu, &view_menu, &edit_menu, &window_menu],
    )
}

const MENU_ACTION_IDS: [&str; 8] = [
    "new",
    "resume",
    "mods",
    "playpause",
    "flatten",
    "share",
    "vwap",
    "sound",
];

fn dispatch_menu_action<R: Runtime>(app: &AppHandle<R>, id: &str) {
    if !MENU_ACTION_IDS.contains(&id) {
        return;
    }
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let Ok(encoded) = serde_json::to_string(id) else {
        return;
    };
    let _ = window.eval(&format!("window.tapeMenu && window.tapeMenu({encoded})"));
}

fn announce_available_update<R: Runtime>(app: AppHandle<R>) {
    tauri::async_runtime::spawn(async move {
        let Ok(updater) = app.updater() else {
            return;
        };
        let Ok(Some(update)) = updater.check().await else {
            return;
        };
        let Some(window) = app.get_webview_window("main") else {
            return;
        };
        let Ok(encoded) = serde_json::to_string(&update.version) else {
            return;
        };
        let _ = window.eval(&format!("window.tapeUpdate && window.tapeUpdate({encoded})"));
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED)
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .menu(build_menu)
        .on_menu_event(|app, event| dispatch_menu_action(app, event.id().as_ref()))
        .setup(|app| {
            announce_available_update(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
