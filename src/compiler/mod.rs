use std::process::Command;
use std::path::Path;
use std::fs;
use std::io;

pub struct CompilerOptions {
    pub target: String,
    pub sdk_path: String,
    pub min_version: String,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            target: "arm64-apple-macos11.0".to_string(),
            sdk_path: "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk".to_string(),
            min_version: "11.0".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum CompileError {
    IoError(io::Error),
    AssemblerError(String),
    LinkerError(String),
}

impl From<io::Error> for CompileError {
    fn from(error: io::Error) -> Self {
        CompileError::IoError(error)
    }
}

pub struct Compiler {
    options: CompilerOptions,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    pub fn compile_and_link(&self, asm_path: &Path) -> Result<(), CompileError> {
        let obj_path = asm_path.with_extension("o");
        let exe_path = asm_path.file_stem().unwrap().to_owned();

        // Run assembler (as)
        let as_output = Command::new("as")
            .arg("-o")
            .arg(&obj_path)
            .arg(asm_path)
            .arg("-arch")
            .arg("arm64")
            .arg("--target")
            .arg(&self.options.target)
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
            .arg(&self.options.sdk_path)
            .arg("-macos_version_min")
            .arg(&self.options.min_version)
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
} 