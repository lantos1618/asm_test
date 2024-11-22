use asm_test::*;
use asm_test::instruction::GenericRegister;
use std::path::Path;
use std::fs;
mod common;

#[test]
fn test_compile_hello_world() {
    let mut program = common::setup_test_program();
    
    // Create a simple hello world program
    program.ins.add(
        GenericRegister::X0,
        GenericRegister::X1,
        GenericRegister::X2
    );
    program.ins.bl("_printf");
    program.ins.mov(GenericRegister::X0, GenericRegister::XZR);
    program.ins.bl("_exit");
    
    // Write and compile
    let asm_path = Path::new("test_hello.s");
    fs::write(&asm_path, program.to_string()).expect("Failed to write assembly");
    
    assert!(program.compile(&asm_path).is_ok());
    
    // Clean up
    fs::remove_file(asm_path).unwrap();
    fs::remove_file(asm_path.with_extension("")).unwrap(); // Remove executable
} 