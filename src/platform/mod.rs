pub mod macos;

pub trait Platform {
    fn function_prefix(&self) -> &'static str;
    fn line_comment(&self) -> &'static str;
    fn data_section(&self) -> &'static str;
    fn text_section(&self) -> &'static str;
} 