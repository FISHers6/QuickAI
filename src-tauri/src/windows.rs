use tauri::Manager;
use tauri::LogicalPosition;
use tauri::PhysicalPosition;

use anyhow::{Result, anyhow};
use crate::AppState;
use crate::select;
use crate::APP;
use mouse_position::mouse_position::Mouse;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use window_shadows::set_shadow;

const CHATGPT_WINDOWS: &'static str = "chatgpt";

fn get_mouse_location() -> Result<(i32, i32)> {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => Ok((x, y)),
        Mouse::Error => Err(anyhow!("")),
   }
}

pub fn chatgpt_windows() {
   let handle = APP.get().unwrap();
   let selected_text = select::selected_text().unwrap_or_default();
   let state: tauri::State<AppState> = handle.state();
   *state.selected_content.write() = selected_text;
   match handle.get_window(CHATGPT_WINDOWS) {
       Some(window) => {
            let (x, y): (i32, i32) = get_mouse_location().unwrap_or_default();
            if cfg!(target_os = "macos") {
                let _ = window
                    .set_position(LogicalPosition::new(x as f64, y as f64));
            } else {
                let _ = window
                    .set_position(PhysicalPosition::new(x as f64, y as f64));
            }

            window.unminimize().unwrap();
            window.set_focus().unwrap();
            window.show().unwrap();
       }
       None => {
           let builder = tauri::WindowBuilder::new(
               handle,
               "chatgpt",
               tauri::WindowUrl::App("index.html".into()),
           )
           .inner_size(480.0, 480.0)
           .always_on_top(true)
           .fullscreen(false)
           .decorations(false)
           .skip_taskbar(true)
           .focused(true)
           .center()
           .title("Ask ChatGPT~");
        
           let window = builder.build().unwrap();

           #[cfg(any(target_os = "macos", target_os = "windows"))]
           {
               set_shadow(&window, true).unwrap_or_default();
           }
       }
    }
}