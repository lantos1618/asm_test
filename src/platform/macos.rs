use super::Platform;

pub struct MacOS;

impl Platform for MacOS {
    fn function_prefix(&self) -> &'static str { "_" }
    fn line_comment(&self) -> &'static str { "//" }
    fn data_section(&self) -> &'static str { ".section __DATA,__data" }
    fn text_section(&self) -> &'static str { ".section __TEXT,__text" }
} 