use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{AppHandle, Manager};
use std::io;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

static IS_DOWN: AtomicBool = AtomicBool::new(false);

use crate::app::window_manager::WindowManager;
use crate::settings::shortcuts::ShortcutConfig;

pub struct ShortcutEngine;

impl ShortcutEngine {
  pub fn init(app: &AppHandle, config: &ShortcutConfig, wm: WindowManager) -> tauri::Result<()> {
    let app_handle = app.clone();
    let wm_hotkey = wm.clone();

    let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);

    app.plugin(
      tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, _shortcut, event| {
        match event.state() {
          ShortcutState::Pressed => {
            if IS_DOWN.swap(true, Ordering::SeqCst) {
              return;
            }

            wm_hotkey.on_scratchpad_hotkey(&app_handle);
          }
          ShortcutState::Released => {
            IS_DOWN.store(false, Ordering::SeqCst);
          }
        }
      })
      .build(),
    )?;

    app.global_shortcut().register(shortcut)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
  }
}
