



pub struct PlatformInput;

#[cfg(target_os = "windows")]
mod windows {
    use super::PlatformInput;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{INPUT_0, VK_RETURN, INPUT_KEYBOARD,KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, INPUT, SendInput, KEYBDINPUT};

    impl PlatformInput {
        pub fn send_content(text: &str) {
            unsafe {
                for ch in text.chars() {
                    let mut input: Vec<INPUT> = Vec::new();

                    if ch == '\n' {
                        input.push(INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: VK_RETURN,
                                    wScan: 0x1C,
                                    dwFlags: KEYEVENTF_UNICODE,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        });
                        input.push(INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: VK_RETURN,
                                    wScan: 0x0A,
                                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        });
                    }else {
                        let event: INPUT = INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: 0,
                                    wScan: ch as u16,
                                    dwFlags: KEYEVENTF_UNICODE,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        };
                        input.push(event);
                
                        let event: INPUT = INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: 0,
                                    wScan: ch as u16,
                                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        };
                        
                        input.push(event);
                    }
                    
                    let result = SendInput(input.len() as u32, input.as_mut_ptr(), std::mem::size_of::<INPUT>() as i32);
                    if result == 0 {
                        tracing::warn!("Failed to send input");
                    }
                    // need sleep wait input event finish
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod mac {

}