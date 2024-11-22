use crate::{Target, Register, Directive, Ins, Operand};

#[derive(Debug, Clone)]
pub enum X86Register {
    RAX,
    RBX,
    RCX,
    RDX,
    // Add more registers as needed
}

impl Register for X86Register {
    fn name(&self) -> &'static str {
        match self {
            X86Register::RAX => "rax",
            X86Register::RBX => "rbx",
            X86Register::RCX => "rcx",
            X86Register::RDX => "rdx",
            // Add more cases as needed
        }
    }
}

#[derive(Debug)]
pub struct X86Target;

impl Target for X86Target {
    fn compile_directive(&self, directive: &Directive) -> String {
        match directive {
            Directive::Global(symbol) => format!("global {}", symbol),
            Directive::Extern(symbol) => format!("extern {}", symbol),
        }
    }

    fn compile_instruction(&self, instruction: &Ins) -> String {
        match instruction {
            Ins::Mov { dest, src } => {
                match src {
                    Operand::Immediate(imm) => format!("mov {}, {}", dest, imm),
                    Operand::Register(reg) => format!("mov {}, {}", dest, reg),
                    Operand::Symbol(sym) => format!("mov {}, [{}]", dest, sym),
                }
            }
            Ins::Bl { symbol } => {
                format!("call {}", symbol)
            }
            Ins::Call { symbol } => {
                format!("call {}", symbol)
            }
            Ins::Adrp { dest, symbol } => {
                format!("lea {}, [{}]", dest, symbol)
            }
            Ins::Add { dest, src, symbol } => {
                format!("lea {}, [{} + {}]", dest, src, symbol)
            }
            Ins::Ret => "ret".to_string(),
            Ins::Nop => "nop".to_string(),
        }
    }

    fn section_data_directive(&self) -> String {
        "section .data".to_string()
    }

    fn section_text_directive(&self) -> String {
        "section .text".to_string()
    }
} 