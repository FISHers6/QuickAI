use crate::easy_thing::{foreground::PlatformForeground, input::PlatformInput};

pub fn search_windows() {
    let foreground_handle = PlatformForeground::get_foreground_window();
    tracing::info!(foreground_handle = foreground_handle);

    let content = "public class Main {\npublic static void main(String[] args) {\nSystem.out.println(\"Hello, world!\");\n}\n}";
    PlatformForeground::set_foreground_window(foreground_handle);
    PlatformInput::send_content(content);
}