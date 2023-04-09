pub struct PlatformForeground;

#[cfg(target_os = "windows")]
mod windows {
    use super::PlatformForeground;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

    impl PlatformForeground {
        pub fn get_foreground_window() -> isize {
            // 获取目标窗口句柄
            // let window_title = "Target Window Title";
            // let wide_title: Vec<u16> = OsStr::new(window_title).encode_wide().chain(Some(0)).collect();
            //获取前台窗口句柄
            let hwnd = unsafe { GetForegroundWindow() };
            let mut pid: u32 = 0;
            let _tid = unsafe { GetWindowThreadProcessId(hwnd, &mut pid as *mut u32) };
            tracing::info!(pid = pid, hwnd = hwnd);
            hwnd
        }

        pub fn set_foreground_window(hwnd: isize) {
            unsafe {
                let current_hwnd = GetForegroundWindow();
                if current_hwnd != hwnd {
                    tracing::info!(current_hwnd = current_hwnd, hwnd = hwnd);
                    SetForegroundWindow(hwnd);
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod mac {}
