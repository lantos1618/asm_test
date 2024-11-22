use lib::*;
mod common;

// Arithmetic Tests
#[test]
fn test_arithmetic_instructions() {
    let mut program = common::setup_test_program();
    
    // Test all arithmetic operations
    program.arch.add(ARM64Register::X0, ARM64Register::X1, ARM64Register::X2);
    program.arch.sub(ARM64Register::X3, ARM64Register::X4, ARM64Register::X5);
    program.arch.mul(ARM64Register::X6, ARM64Register::X7, ARM64Register::X8);
    program.arch.fadd(ARM64Register::V0, ARM64Register::V1, ARM64Register::V2);
    
    let instructions = program.arch.get_instructions();
    assert_eq!(instructions.len(), 4);
    
    // Verify each instruction
    match &instructions[0] {
        arch::arm64::Instruction::Arithmetic(
            arch::arm64::ArithmeticOp::Add { dst, src1, src2 }
        ) => {
            assert_eq!(*dst, ARM64Register::X0);
            assert_eq!(*src1, ARM64Register::X1);
            assert_eq!(*src2, ARM64Register::X2);
        },
        _ => panic!("Expected Add instruction"),
    }
    
    // ... similar checks for other instructions
}

// Branch Tests
#[test]
fn test_branch_instructions() {
    let mut program = common::setup_test_program();
    
    program.arch.bl("external_func");
    program.arch.b("local_label");
    program.arch.ret();
    program.arch.cbz(ARM64Register::X0, "zero_branch");
    
    let instructions = program.arch.get_instructions();
    assert_eq!(instructions.len(), 4);
    
    // Verify branch instructions
    match &instructions[0] {
        arch::arm64::Instruction::Branch(
            arch::arm64::BranchOp::Bl { label }
        ) => {
            assert_eq!(label, "external_func");
        },
        _ => panic!("Expected Bl instruction"),
    }
    // ... verify other branch instructions
}

// Register Tests
#[test]
fn test_register_display() {
    assert_eq!(ARM64Register::X0.to_string(), "x0");
    assert_eq!(ARM64Register::V0.to_string(), "v0");
    assert_eq!(ARM64Register::SP.to_string(), "sp");
    assert_eq!(ARM64Register::LR.to_string(), "lr");
    assert_eq!(ARM64Register::XZR.to_string(), "xzr");
}

// Complex Instruction Sequences
#[test]
fn test_function_prologue() {
    let mut program = common::setup_test_program();
    
    // Typical function prologue
    program.arch.sub(ARM64Register::SP, ARM64Register::SP, ARM64Register::X16); // Adjust stack
    program.arch.str(ARM64Register::LR, "[SP, #-16]!"); // Store link register
    
    let instructions = program.arch.get_instructions();
    assert_eq!(instructions.len(), 2);
    // ... verify instructions
} 