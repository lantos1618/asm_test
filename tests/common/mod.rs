// Shared test utilities
use lib::*;

pub fn setup_test_program() -> Program<ARM64, ARM64Register, MacOS> {
    Program::new(ARM64::new(), MacOS)
} 