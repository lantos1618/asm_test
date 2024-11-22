use asm_test::*;
use asm_test::platform::macos::MacOS;
use asm_test::platform::Platform;
mod common;

#[test]
fn test_macos_platform_specifics() {
    let platform = MacOS;
    
    assert_eq!(platform.function_prefix(), "_");
    assert_eq!(platform.text_section(), ".section __TEXT,__text");
    assert_eq!(platform.data_section(), ".section __DATA,__data");
    assert_eq!(platform.line_comment(), "//");
} 