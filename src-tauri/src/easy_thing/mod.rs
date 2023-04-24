use tauri::Manager;

pub mod foreground;
pub mod input;


pub fn send_auto_input_value(content: String) -> Result<(), String> {
    let handle = crate::APP.get().unwrap();
    let state: tauri::State<crate::AppState> = handle.state();
    crate::tauri_windows::search::show_foreground_window();
    let answer_sender = crate::command::get_or_init_auto_input(&state);
    answer_sender
        .send(content)
        .map_err(|err| format!("{:?}", err))?;
    Ok(())
}
