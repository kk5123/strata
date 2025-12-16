mod app;
mod settings;

use app::{window_manager::WindowManager, shortcuts::ShortcutEngine};
use settings::shortcuts::ShortcutConfig;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn scratchpad_closed() {
  println!("Scratchpad closed.")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      #[cfg(desktop)]
      {
        let wm = WindowManager::init(app.handle());

        let config = ShortcutConfig {
          toggle_scratchpad: "Ctrl+Shift+Space".into(),
        };

        ShortcutEngine::init(app.handle(), &config, wm)?;

        Ok(())
      }
    })
    .invoke_handler(tauri::generate_handler![greet, scratchpad_closed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
