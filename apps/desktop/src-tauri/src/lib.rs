mod app;
use app::window_manager::WindowManager;

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;

static IS_DOWN: AtomicBool = AtomicBool::new(false);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      #[cfg(desktop)]
      {
        let wm = WindowManager::new();

        use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
        let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);

        let wm_hotkey = wm.clone();
        let app_for_wm = app.handle().clone();
        app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, _shortcut, event| {
            match event.state() {
              ShortcutState::Pressed => {
                if IS_DOWN.swap(true, Ordering::SeqCst) {
                  return;
                }

                wm_hotkey.on_scratchpad_hotkey(&app_for_wm);
              }
              ShortcutState::Released => {
                IS_DOWN.store(false, Ordering::SeqCst);
              }
            }
          })
          .build(),
        )?;

        app.global_shortcut().register(shortcut)?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
