use asm_test::*;
use asm_test::arch::arm64::ARM64;
use asm_test::instruction::GenericRegister;
use std::path::Path;
use std::fs;

fn main() {
    let mut program = Program::new(ARM64::new());
    
    // Add string variable to data section
    let msg_label = program.var("hello_msg", "Hello, World!\n");
    
    // The actual program with comments
    program.ins
        .comment("Load address of hello string")
        .adrp(GenericRegister::X0, &msg_label)
        .comment("Add page offset")
        .add(GenericRegister::X0, GenericRegister::X0, format!("{}@PAGEOFF", msg_label))
        .comment("Call printf")
        .bl("_printf")
        .comment("Set return code to 0")
        .mov(GenericRegister::X0, GenericRegister::XZR)
        .comment("Exit program")
        .bl("_exit");
    
    // Write assembly to file
    let asm_path = Path::new("program.s");
    fs::write(&asm_path, program.to_string()).expect("Failed to write assembly");
    
    // Compile and link
    program.compile(&asm_path).expect("Failed to compile program");
}