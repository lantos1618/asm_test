use lib::*;
mod common;

#[test]
fn test_hello_world_program() {
    let mut program = common::setup_test_program();
    
    // Build a hello world program
    program.arch.add(ARM64Register::X0, ARM64Register::X1, ARM64Register::X2);
    program.arch.bl("_printf");
    program.arch.mov(ARM64Register::X0, ARM64Register::XZR);
    program.arch.bl("_exit");
    
    let instructions = program.arch.get_instructions();
    assert_eq!(instructions.len(), 4);
    // ... verify instruction sequence
}

#[test]
fn test_function_call_sequence() {
    let mut program = common::setup_test_program();
    
    // Test function call with parameter setup
    program.arch.mov(ARM64Register::X0, ARM64Register::X1);  // First parameter
    program.arch.mov(ARM64Register::X1, ARM64Register::X2);  // Second parameter
    program.arch.bl("_function");
    program.arch.mov(ARM64Register::X0, ARM64Register::X0);  // Preserve return value
    
    let instructions = program.arch.get_instructions();
    assert_eq!(instructions.len(), 4);
    // ... verify instruction sequence
}

#[test]
fn test_floating_point_operations() {
    let mut program = common::setup_test_program();
    
    // Test floating point operation sequence
    program.arch.fadd(ARM64Register::V0, ARM64Register::V1, ARM64Register::V2);
    program.arch.str(ARM64Register::V0, "[X0]");
    
    let instructions = program.arch.get_instructions();
    assert_eq!(instructions.len(), 2);
    // ... verify instruction sequence
} 