use asm_test::*;
use asm_test::instruction::GenericRegister;
mod common;

#[test]
fn test_hello_world_program() {
    let mut program = common::setup_test_program();
    
    // Build a hello world program
    program.ins.add(
        GenericRegister::X0,
        GenericRegister::X1,
        GenericRegister::X2
    );
    program.ins.bl("_printf");
    program.ins.mov(GenericRegister::X0, GenericRegister::XZR);
    program.ins.bl("_exit");
    
    let instructions = program.ins.arch.get_instructions();
    assert_eq!(instructions.len(), 4);
}

#[test]
fn test_function_call_sequence() {
    let mut program = common::setup_test_program();
    
    // Test function call with parameter setup
    program.ins.mov(GenericRegister::X0, GenericRegister::X1);
    program.ins.mov(GenericRegister::X1, GenericRegister::X2);
    program.ins.bl("_function");
    
    let instructions = program.ins.arch.get_instructions();
    assert_eq!(instructions.len(), 3);
} 