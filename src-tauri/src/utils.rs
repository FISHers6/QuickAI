use anyhow::Result;
#[cfg(not(target_os = "macos"))]
pub fn get_screen_size() -> Result<(f64, f64)> {
    let event_loop = winit::event_loop::EventLoop::new();
    // 获取主监视器信息
    let screen_size = match event_loop.primary_monitor() {
        Some(monitor) => {
            let _ = monitor.scale_factor();
            let size = monitor.size();
            (size.width as f64, size.height as f64)
        }
        None => {
            tracing::info!("get screen size error");
            (1920.0, 1080.0)
        }
    };
    Ok(screen_size)
}

#[cfg(target_os = "macos")]
pub fn get_screen_size() -> Result<(f64, f64)> {
    use cocoa::{base::id, foundation::NSRect};
    use objc::{msg_send, runtime::Class, sel, sel_impl};

    let screen_size = unsafe {
        let screens: id = msg_send![Class::get("NSScreen").unwrap(), screens];
        let main_screen: id = msg_send![screens, objectAtIndex:0];
        let screen_rect: NSRect = msg_send![main_screen, frame];
        (
            screen_rect.size.width as f64,
            screen_rect.size.height as f64,
        )
    };
    Ok(screen_size)
}
