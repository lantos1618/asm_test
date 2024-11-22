// Shared test utilities
use asm_test::*;
use asm_test::arch::arm64::{ARM64, Arm64Register};

pub fn setup_test_program() -> Program<ARM64, Arm64Register> {
    Program::new(ARM64::new())
} 