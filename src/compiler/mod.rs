use std::process::Command;
use std::path::Path;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub enum Architecture {
    ARM64,
    X86_64,
}

impl Architecture {
    fn as_str(&self) -> &'static str {
        match self {
            Architecture::ARM64 => "arm64",
            Architecture::X86_64 => "x86_64",
        }
    }
}

pub struct CompilerOptions {
    pub target: String,
    pub sdk_path: String,
    pub min_version: String,
    pub architecture: Architecture,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            target: "arm64-apple-macos11.0".to_string(),
            sdk_path: "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk".to_string(),
            min_version: "11.0".to_string(),
            architecture: Architecture::ARM64,
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