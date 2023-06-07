use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::tauri_windows::select::SELECT_WINDOWS_HEIGHT;
use crate::tauri_windows::select::SELECT_WINDOWS_WIDTH;
use crate::AppState;
use anyhow::{anyhow, Result};
use rdev::{listen, Button, Event, EventType};
use tauri::AppHandle;
use tauri::Manager;
use tauri::State;

pub fn register_task(handle: &AppHandle) {
    let state: tauri::State<AppState> = handle.state();
    let _ = state.spawn_task(|| {
        if let Ok((mouse_position_x, mouse_position_y)) = get_mouse_position() {
            let mut select_listen = SelectListener {
                last_mouse_position_x: mouse_position_x,
                last_mouse_position_y: mouse_position_y,
                last_press_mouse_time: Instant::now(),
            };
            if let Err(error) = listen(move |event: Event| match event.event_type {
                EventType::ButtonPress(Button::Left) => {
                    if let Ok((mouse_position_x, mouse_position_y)) = get_mouse_position() {
                        select_listen.last_mouse_position_x = mouse_position_x;
                        select_listen.last_mouse_position_y = mouse_position_y;
                    }
                }
                EventType::ButtonRelease(Button::Left) => {
                    if let Ok((current_mouse_position_x, current_mouse_position_y)) =
                        get_mouse_position()
                    {
                        if select_listen
                            .should_select(current_mouse_position_x, current_mouse_position_y)
                        {
                            select_listen.select_content_and_show_tip(
                                current_mouse_position_x as f64,
                                current_mouse_position_y as f64,
                            );
                        }
                        select_listen.last_press_mouse_time = Instant::now();
                        select_listen.last_mouse_position_x = current_mouse_position_x;
                        select_listen.last_mouse_position_y = current_mouse_position_y;
                    }
                }
                _ => {}
            }) {
                tracing::error!("listen mouse error: {:?}", error);
            }
        } else {
            tracing::error!("get_mouse_position failed");
        }
    });
}

const MIN_DISTANCE_TIME: u128 = 100;
const MIN_DISTANCE_POSITION: f64 = 10.0;
const WINDOWS_POSITION_OFFSET: f64 = 15.0;

use mouse_position::mouse_position::Mouse;

fn get_mouse_position() -> Result<(f64, f64)> {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => Ok((x as f64, y as f64)),
        Mouse::Error => Err(anyhow!("get mouse position failed")),
    }
}

struct SelectListener {
    last_mouse_position_x: f64,
    last_mouse_position_y: f64,
    last_press_mouse_time: Instant,
}

impl SelectListener {
    fn should_select(&self, mouse_position_x: f64, mouse_position_y: f64) -> bool {
        fn is_enable_select() -> bool {
            if let Some(app_handle) = crate::APP.get() {
                let state: State<AppState> = app_handle.state();
                state.enable_select.load(Ordering::SeqCst)
            } else {
                false
            }
        }
        self.last_press_mouse_time.elapsed().as_millis() >= MIN_DISTANCE_TIME
            && ((mouse_position_x - self.last_mouse_position_x).abs() >= MIN_DISTANCE_POSITION
                || (mouse_position_y - self.last_mouse_position_y).abs() >= MIN_DISTANCE_POSITION)
            && is_enable_select()
    }

    fn select_content_and_show_tip(&self, mouse_position_x: f64, mouse_position_y: f64) {
        if let Ok(selected_content) = crate::select::get_selected_text() {
            let trimed_selected_content = selected_content.trim();
            if !trimed_selected_content.is_empty() {
                tracing::info!(selected_content = trimed_selected_content);
                if let Some(handle) = crate::APP.get() {
                    let state: State<AppState> = handle.state();
                    *state.selected_content.write() = trimed_selected_content.to_string();
                    let mut position_y = if self.last_mouse_position_y < mouse_position_y {
                        self.last_mouse_position_y
                    } else {
                        mouse_position_y
                    };
                    position_y = position_y - SELECT_WINDOWS_HEIGHT - WINDOWS_POSITION_OFFSET;
                    if position_y < SELECT_WINDOWS_HEIGHT {
                        position_y = SELECT_WINDOWS_HEIGHT;
                    }
                    let mut position_x = (self.last_mouse_position_x + mouse_position_x) / 2.0
                        - SELECT_WINDOWS_WIDTH / 2.0;
                    let mut min_position_x = if self.last_mouse_position_x > mouse_position_x {
                        mouse_position_x
                    } else {
                        self.last_mouse_position_x
                    };
                    if min_position_x < SELECT_WINDOWS_WIDTH / 2.0 {
                        min_position_x = SELECT_WINDOWS_WIDTH / 2.0;
                    }
                    if position_x < min_position_x {
                        position_x = min_position_x;
                    }
                    crate::tauri_windows::select::build_select_windows(
                        handle,
                        trimed_selected_content,
                        position_x,
                        position_y,
                    );
                }
            }
        }
    }
}
