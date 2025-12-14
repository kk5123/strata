use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

static IS_DOWN: AtomicBool = AtomicBool::new(false);

const STRATUM_LABEL: &str = "Stratum";

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
        if app.get_webview_window(STRATUM_LABEL).is_none() {
          WebviewWindowBuilder::new(app, STRATUM_LABEL, WebviewUrl::App("/".into()))
            .title(STRATUM_LABEL)
            .visible(false)
            .decorations(false)
            .resizable(true)
            .always_on_top(true)
            .inner_size(480.0, 320.0)
            .build()?;
        }

        let handle = app.handle().clone();

        use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

        let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);
        app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, _shortcut, event| {
            match event.state() {
              ShortcutState::Pressed => {
                if IS_DOWN.swap(true, Ordering::SeqCst) {
                  return;
                }

                if let Some(win) = handle.get_webview_window(STRATUM_LABEL) {
                  let _ = win.show();
                  let _ = win.set_focus();
                }
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
