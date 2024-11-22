use asm_test::*;
use asm_test::arch::arm64::{ARM64, Arm64Register};
use asm_test::instruction::{GenericRegister, ArithmeticBuilder, BranchBuilder, MovBuilder, LoadStoreBuilder, Operand};
use instruction::RegisterMapping;
mod common;

#[test]
fn test_arithmetic_instructions() {
    let mut program = common::setup_test_program();
    
    // Test all arithmetic operations
    program.ins.add(
        GenericRegister::X0,
        GenericRegister::X1,
        GenericRegister::X2
    );
    
    let instructions = program.ins.arch.get_instructions();
    assert_eq!(instructions.len(), 1);
    
    // Verify instruction
    match &instructions[0] {
        arch::arm64::Instruction::Arithmetic(
            arch::arm64::ArithmeticOp::Add { dst, src1, src2 }
        ) => {
            assert_eq!(*dst, GenericRegister::X0.to_arch_reg());
            assert_eq!(*src1, GenericRegister::X1.to_arch_reg());
            assert_eq!(*src2, GenericRegister::X2.to_arch_reg());
        },
        _ => panic!("Expected Add instruction"),
    }
}

#[test]
fn test_branch_instructions() {
    let mut program = common::setup_test_program();
    
    program.ins.bl("external_func");
    program.ins.bl("local_label");
    
    let instructions = program.ins.arch.get_instructions();
    assert_eq!(instructions.len(), 2);
    
    // Verify branch instructions
    match &instructions[0] {
        arch::arm64::Instruction::Branch(
            arch::arm64::BranchOp::Bl { label }
        ) => {
            assert_eq!(label, "external_func");
        },
        _ => panic!("Expected Bl instruction"),
    }
}

#[test]
fn test_function_prologue() {
    let mut program = common::setup_test_program();
    
    // Typical function prologue
    program.ins.add(
        GenericRegister::SP,
        GenericRegister::SP,
        "-16"
    );
    
    program.ins.mov(GenericRegister::X0, GenericRegister::LR);
    
    let instructions = program.ins.arch.get_instructions();
    assert_eq!(instructions.len(), 2);
} 