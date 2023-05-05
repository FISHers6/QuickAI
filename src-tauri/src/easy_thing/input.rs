use anyhow::Result;

pub struct CrossformInput;

impl CrossformInput {
    pub fn auto_input_text_using_copy(text: &str) -> Result<()> {
        if text.contains('\n') {
            // 分割字符串
            let mut splits = text.split('\n').peekable();
            while let Some(line) = splits.next() {
                // 输入一行文本
                crate::select::copy_and_paste(line.to_string())?;
                if splits.peek().is_some() {
                    // 如果不是最后一行则输入回车换行符
                    std::thread::sleep(std::time::Duration::from_millis(30));
                    crate::select::press_enter();
                }
            }
        } else {
            crate::select::copy_and_paste(text.to_string())?
        };
        Ok(())
    }
}
