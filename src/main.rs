use asm_test::*;
use asm_test::arch::arm64::ARM64;
use asm_test::instruction::GenericRegister;
use std::path::Path;
use std::fs;
use std::process::Command;

fn compile_and_link(asm_path: &Path, options: &CompilerOptions) -> Result<(), CompileError> {
    let obj_path = asm_path.with_extension("o");
    let exe_path = asm_path.file_stem().unwrap().to_owned();

    // Run assembler (as)
    let as_output = Command::new("as")
        .arg("-o")
        .arg(&obj_path)
        .arg(asm_path)
        .arg("-arch")
        .arg(options.architecture.as_str())
        .arg("--target")
        .arg(&options.target)
        .output()?;

    if !as_output.status.success() {
        return Err(CompileError::AssemblerError(
            String::from_utf8_lossy(&as_output.stderr).to_string()
        ));
    }

    // Run linker (ld)
    let ld_output = Command::new("ld")
        .arg("-o")
        .arg(&exe_path)
        .arg(&obj_path)
        .arg("-lSystem")
        .arg("-syslibroot")
        .arg(&options.sdk_path)
        .arg("-macos_version_min")
        .arg(&options.min_version)
        .arg("-e")
        .arg("_start")
        .output()?;

    if !ld_output.status.success() {
        return Err(CompileError::LinkerError(
            String::from_utf8_lossy(&ld_output.stderr).to_string()
        ));
    }

    // Clean up object file
    fs::remove_file(obj_path)?;

    Ok(())
}

fn main() {
    let options = CompilerOptions::default();
    
    let mut program = Program::new(ARM64::new());


    let data_section = program.section("data");
    let text_section = program.section("text");
    
    
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
    compile_and_link(&asm_path, &options).expect("Failed to compile program");
}