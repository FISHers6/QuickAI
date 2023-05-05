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
mod mac {
    use super::PlatformForeground;
    use std::process::{Command, Output};

    impl PlatformForeground {
        pub fn run_script(script: String) -> Result<String, String> {
            let output: Output = Command::new("osascript")
                .args(&["-e", &script])
                .output()
                .map_err(|_| "Failed to execute command".to_string())?;

            if !output.status.success() {
                return Err(String::from_utf8_lossy(output.stderr.as_slice()).into());
            }

            Ok(String::from_utf8_lossy(output.stdout.as_slice())
                .trim()
                .to_owned())
        }

        pub fn get_foreground_window() -> Result<isize, String> {
            let script = r#"tell application "System Events"
                                set frontApp to name of first process whose frontmost is true
                                set frontWinID to id of front window of (first application process whose frontmost is true)
                                return frontWinID as string
                            end tell "#;
            let hwnd = PlatformForeground::run_script(String::from(script))?;
            hwnd.parse::<isize>()
                .map_err(|_err| format!("parse hwnd error"))
        }

        pub fn set_foreground_window(window_id: isize) -> Result<(), String> {
            let script = format!(
                "tell application \"System Events\" to tell window id {} of (first application process whose frontmost is true)\n\
                 if current tab is not missing value then perform action \"AXRaise\"\n\
                 perform action \"AXRaise\"\n\
                 end tell",
                window_id
            );

            PlatformForeground::run_script(script).map(|_| ())
        }
    }
}
