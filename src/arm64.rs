use crate::{Target, Register, Directive, Ins, Operand};

#[derive(Debug, Clone)]
pub enum Arm64Register {
    X0,
    X1,

}

impl Register for Arm64Register {
    fn name(&self) -> &'static str {
        match self {
            Arm64Register::X0 => "x0",
            Arm64Register::X1 => "x1",

        }
    }
}

#[derive(Debug)]
pub struct ARM64Target;

impl Target for ARM64Target {
    fn compile_directive(&self, directive: &Directive) -> String {
        match directive {
            Directive::Global(symbol) => format!(".global {}", symbol),
            Directive::Extern(symbol) => format!(".extern {}", symbol),
        }
    }

    fn compile_instruction(&self, instruction: &Ins) -> String {
        match instruction {
            Ins::Mov { dest, src } => {
                match src {
                    Operand::Immediate(imm) => format!("mov {}, #{}", dest, imm),
                    Operand::Register(reg) => format!("mov {}, {}", dest, reg),
                    Operand::Symbol(sym) => format!("mov {}, {}", dest, sym),
                }
            }
            Ins::Bl { symbol } => {
                format!("bl {}", symbol)
            }
            Ins::Adrp { dest, symbol } => {
                format!("adrp {}, {}", dest, symbol)
            }
            Ins::Add { dest, src, symbol } => {
                format!("add {}, {}, {}", dest, src, symbol)
            }
            Ins::Call { symbol } => {
                format!("bl {}", symbol)
            }
            Ins::Ret => "ret".to_string(),
            Ins::Nop => "nop".to_string(),
        }
    }

    fn section_data_directive(&self) -> String {
        ".section __DATA,__data".to_string()
    }

    fn section_text_directive(&self) -> String {
        ".section __TEXT,__text".to_string()
    }
} 
