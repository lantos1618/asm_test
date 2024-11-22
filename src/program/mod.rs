use crate::{builder::InstructionBuilder, instruction::Register};
use crate::context::Context;
use std::fmt;
use std::path::Path;
// use crate::compiler::{Compiler, CompilerOptions, CompileError};
use crate::compiler::{CompilerOptions, CompileError};
use crate::instruction::{GenericRegister, RegisterMapping};

pub struct Program<A, R: Register> {
    pub ins: InstructionBuilder<A, R>,
    pub ctx: Context,
}

impl<A, R: Register> Program<A, R> 
where
    GenericRegister: RegisterMapping<R>
{
    pub fn new(arch: A) -> Self {
        Self {
            ins: InstructionBuilder::<A, R>::new(arch),
            ctx: Context::new(),
        }
    }

    pub fn var(&mut self, name: &str, value: &str) -> String {
        self.ctx.add_variable(name, value)
    }

    // pub fn compile(&self, path: &Path) -> Result<(), CompileError> {
    //     let compiler = Compiler::new(CompilerOptions::default());
    //     compiler.compile_and_link(path)
    // }
}

impl<A, R: Register> fmt::Display for Program<A, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write data section
        writeln!(f, ".section __DATA,__data")?;
        for var in self.ctx.variables.values() {
            writeln!(f, "{}:", var.label)?;
            writeln!(f, "    .asciz \"{}\"", var.value)?;
        }
        
        // Write text section
        writeln!(f, ".section __TEXT,__text")?;
        writeln!(f, ".global _start")?;
        writeln!(f, "_start:")?;
        
        // TODO: Format instructions with comments
        
        Ok(())
    }
} 