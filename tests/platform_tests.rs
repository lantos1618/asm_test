use lib::*;
mod common;

#[test]
fn test_macos_platform_specifics() {
    let platform = MacOS;
    
    // Test all platform-specific formatting
    assert_eq!(platform.function_prefix(), "_");
    assert_eq!(platform.text_section(), ".section __TEXT,__text");
    assert_eq!(platform.data_section(), ".section __DATA,__data");
    assert_eq!(platform.line_comment(), "//");
}

#[test]
fn test_platform_function_names() {
    let mut program = common::setup_test_program();
    
    // Test platform-specific function name handling
    program.arch.bl("printf");  // Should become _printf on MacOS
    program.arch.bl("malloc");  // Should become _malloc on MacOS
    
    let instructions = program.arch.get_instructions();
    match &instructions[0] {
        arch::arm64::Instruction::Branch(
            arch::arm64::BranchOp::Bl { label }
        ) => {
            assert!(label.starts_with("_"), "MacOS functions should start with underscore");
        },
        _ => panic!("Expected Bl instruction"),
    }
} 