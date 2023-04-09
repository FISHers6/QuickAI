pub struct PlatformInput;

#[cfg(target_os = "windows")]
mod windows {
    use super::PlatformInput;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, MapVirtualKeyW,
        VK_RETURN, 
    };
    
    use anyhow::{anyhow, Result};
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;
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
                    } else {
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

                    let result = SendInput(
                        input.len() as u32,
                        input.as_mut_ptr(),
                        std::mem::size_of::<INPUT>() as i32,
                    );
                    if result == 0 {
                        tracing::warn!("Failed to send input");
                    }
                }
            }
        }
        pub fn send_content_v2(text: &str) {
            unsafe {
                let mut input: Vec<INPUT> = Vec::new();
                for ch in text.chars() {
                    if ch == '\n' {
                        input.push(INPUT {
                            r#type: INPUT_KEYBOARD,
                            Anonymous: INPUT_0 {
                                ki: KEYBDINPUT {
                                    wVk: VK_RETURN as u16,
                                    wScan: MapVirtualKeyW(VK_RETURN as u32, 0) as u16,
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
                                    wVk: VK_RETURN as u16,
                                    wScan: MapVirtualKeyW(VK_RETURN as u32, 0) as u16,
                                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                                    time: 0,
                                    dwExtraInfo: 0,
                                },
                            },
                        });
                    } else {
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
                }
        
                let result = SendInput(
                    input.len() as u32,
                    input.as_mut_ptr(),
                    std::mem::size_of::<INPUT>() as i32,
                );
                if result == 0 {
                    tracing::warn!("Failed to send input");
                }
            }
        }

        pub fn auto_input_text_using_copy(text: &str) -> Result<()> {
            if text.contains('\n') {
                // 分割字符串
                let mut splits = text.split('\n').peekable();
                while let Some(line) = splits.next() {
                    // 输入一行文本
                    Self::copy_and_paste(line.to_string())?;
                    if splits.peek().is_some() {
                        // 如果不是最后一行则输入回车换行符
                        unsafe {
                            let mut input: Vec<INPUT> = Vec::new();
                            input.push(INPUT {
                                r#type: INPUT_KEYBOARD,
                                Anonymous: INPUT_0 {
                                    ki: KEYBDINPUT {
                                        wVk: VK_RETURN as u16,
                                        wScan: MapVirtualKeyW(VK_RETURN as u32, 0) as u16,
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
                                        wVk: VK_RETURN as u16,
                                        wScan: MapVirtualKeyW(VK_RETURN as u32, 0) as u16,
                                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                                        time: 0,
                                        dwExtraInfo: 0,
                                    },
                                },
                            });
                            std::thread::sleep(std::time::Duration::from_millis(30));
                            let result = SendInput(
                                input.len() as u32,
                                input.as_mut_ptr(),
                                std::mem::size_of::<INPUT>() as i32,
                            );
                            if result == 0 {
                                tracing::warn!("Failed to send input");
                            }
                        }
                    }
                }
            } else {
                Self::copy_and_paste(text.to_string())?
            };
            Ok(())
        }

        pub fn copy_and_paste(text: String) -> Result<()> {
            let mut cli_pboard: ClipboardContext =
            ClipboardProvider::new().map_err(|_err| anyhow!("get clipboard error"))?;
             let old_text = cli_pboard
                .get_contents()
                .map_err(|_err| anyhow!("get clipboard content error"))?;
            if let Ok(_) = cli_pboard.set_contents(text) {
                std::thread::sleep(std::time::Duration::from_millis(30));
                crate::select::paste();
                // 将文本粘贴到当前焦点窗口中
                cli_pboard.set_contents(old_text).map_err(|_err| anyhow!("set old clipboard error"))?;
            }
            Ok(())
        }

    }
}

#[cfg(target_os = "macos")]
mod mac {}
