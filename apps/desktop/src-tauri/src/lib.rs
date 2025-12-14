use std::sync::atomic::{AtomicBool, Ordering};

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
        use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

        let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);
        app.handle().plugin(
          tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, _shortcut, event| {
            match event.state() {
              ShortcutState::Pressed => {
                if IS_DOWN.swap(true, Ordering::SeqCst) {
                  return;
                }
                println!("Ctrl+Shift+Space Pressed")
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
