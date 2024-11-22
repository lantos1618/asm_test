use lib::*;

fn main() {
    let mut program = Program::new(ARM64::new(), MacOS);
    
    // Low-level instruction access
    program.arch.add(ARM64Register::X0, ARM64Register::X1, ARM64Register::X2);
    program.arch.fadd(ARM64Register::V0, ARM64Register::V1, ARM64Register::V2);
    
    // High-level operations
    program.add_float_numbers(ARM64Register::V0, ARM64Register::V1, ARM64Register::V2);
    
    // Platform-aware function calls
    program.call_external("printf"); // Will automatically add "_" prefix on MacOS
}