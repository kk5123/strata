use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub const LABEL_DASHBOARD: &str = "main";
pub const LABEL_SCRATCHPAD: &str = "scratchpad";

#[derive(Clone, Debug)]
pub struct WindowManager {
  dashboard_label: &'static str,
  scratchpad_label: &'static str,
}

impl Default for WindowManager {
  fn default() -> Self {
    Self {
      dashboard_label: LABEL_DASHBOARD,
      scratchpad_label: LABEL_SCRATCHPAD,
    }
  }
}

impl WindowManager {
  pub fn init(app: &AppHandle) -> Self {
    let dashboard = app
      .get_webview_window(LABEL_DASHBOARD)
      .expect("dashboard window not found");

    let wm = Self::default();
    let wm_for_event = wm.clone();
    let app_handle = app.clone();

    dashboard.on_window_event(move |event| {
      match event {
        tauri::WindowEvent::Focused(true) => {
          wm_for_event.hide_scratchpad(&app_handle);
        }

        _ => {}
      }
    });

    wm
  }

  pub fn on_scratchpad_hotkey(&self, app: &AppHandle) {
    if !self.is_dashboard_visible(app) {
      if self.is_scratchpad_focused(app) {
        self.hide_scratchpad(app);
      } else {
        self.show_scratchpad(app);
      }
    }
  }

  // --------------------
  // Scratchpad window
  // --------------------

  pub fn show_scratchpad(&self, app: &AppHandle) {
    let win = self.ensure_scratchpad(app);
    let _ = win.show();
    let _ = win.set_focus();
  }

  pub fn hide_scratchpad(&self, app: &AppHandle) {
    if let Some(win) = self.scratchpad(app) {
      let _ = win.hide();
    }
  }

  fn ensure_scratchpad(&self, app: &AppHandle) -> WebviewWindow {
    if let Some(win) = self.scratchpad(app) {
      return win;
    }

    let url = WebviewUrl::App("/#/scratchpad".into());

    WebviewWindowBuilder::new(app, self.scratchpad_label, url)
      .title("Strata")
      .visible(false)
      .decorations(false)
      .resizable(false)
      .always_on_top(false)
      .skip_taskbar(true)
      .build()
      .expect("failed to build scratchpad window")
  }

  fn is_dashboard_visible(&self, app: &AppHandle) -> bool {
    if let Some(win) = self.dashboard(app) {
      if let Ok(false) = win.is_visible() {
        return false;
      }
      if let Ok(true) = win.is_minimized() {
        return false;
      }

      return true;
    }

    false
  }

  fn is_scratchpad_focused(&self, app: &AppHandle) -> bool {
    if let Some(win) = self.scratchpad(app) {
      if let Ok(true) = win.is_focused() {
        return true;
      }
    }

    false
  }

  fn dashboard(&self, app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(self.dashboard_label)
  }

  fn scratchpad(&self, app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(self.scratchpad_label)
  }
}
