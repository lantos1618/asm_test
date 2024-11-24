use serde::{Serialize, Deserialize};
use std::fmt::Display;

use crate::*;

/// Optional comment
type Comment = Option<String>;

/// Enums for macOS ARM64 assembly directives
#[derive(Debug, Serialize, Deserialize)]
pub enum MacOSDirective {
    Global(String),
    Extern(String),
    Section(MacOSSectionType),
}

/// Enum for macOS section types
#[derive(Debug, Serialize, Deserialize)]
pub enum MacOSSectionType {
    Data,
    Text,
    Const,
    Bss,
}

/// Enum for macOS ARM64 registers
#[derive(Debug, Serialize, Deserialize)]
pub enum MacOSRegister {
    /// General-purpose register (e.g., x0-x30 for ARM64)
    X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31,
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V30, V31,
    W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11, W12, W13, W14, W15, W16, W17, W18, W19, W20, W21, W22, W23, W24, W25, W26, W27, W28, W29, W30, W31,
    /// Special registers
    Sp, // Stack Pointer
    Pc, // Program Counter
    Lr, // Link Register
}

/// Enum for macOS ARM64 assembly operations
#[derive(Debug, Serialize, Deserialize)]
pub enum MacOSOp {
    Adrp,
    Add,
    Bl,
    Mov,
    Call,
    Ret,
    Sub,  // Subtract
    Mul,  // Multiply
    Div,  // Divide
    And,  // Bitwise AND
    Or,   // Bitwise OR
    Xor,  // Bitwise XOR
    // Add more operations as needed
}

/// Enum for operands in macOS ARM64 instructions
#[derive(Debug, Serialize, Deserialize)]
pub enum MacOSOperand {
    Register(MacOSRegister),
    Immediate(i64),
    Label(String),
    MemoryAddress(String),
    MemoryOffset { base: MacOSRegister, offset: i64 },
    ScaledIndex { base: MacOSRegister, index: MacOSRegister, scale: u8 },
    // Add more operand types as needed
}

/// Struct for macOS ARM64 assembly instructions
#[derive(Debug, Serialize, Deserialize)]
pub struct MacOSIns {
    pub op: MacOSOp,
    pub operands: Vec<MacOSOperand>,
    pub comment: Comment,
}

/// Enum for label variants in macOS ARM64
#[derive(Debug, Serialize, Deserialize)]
pub enum MacOSLabelVariant {
    Data(MacOSDataLabel),
    Code(MacOSCodeLabel),
}

/// Struct for labels in macOS ARM64
#[derive(Debug, Serialize, Deserialize)]
pub struct MacOSLabel {
    pub val: MacOSLabelVariant,
    pub comment: Comment,
}

/// Struct for code labels in macOS ARM64 code sections
#[derive(Debug, Serialize, Deserialize)]
pub struct MacOSCodeLabel {
    pub name: String,
    pub instructions: Vec<MacOSIns>,
}

/// Struct for data labels in macOS ARM64 data sections
#[derive(Debug, Serialize, Deserialize)]
pub struct MacOSDataLabel {
    pub name: String,
    pub directive: String,
    pub value: String,
}

/// Struct for sections in the macOS ARM64 program
#[derive(Debug, Serialize, Deserialize)]
pub struct MacOSSection {
    pub section_type: MacOSSectionType,
    pub labels: Vec<MacOSLabel>,
}

/// Struct for the entire macOS ARM64 assembly program
#[derive(Debug, Serialize, Deserialize)]
pub struct MacOSProgram {
    pub directives: Vec<MacOSDirective>,
    pub sections: Vec<MacOSSection>,
}

impl From<Directive> for MacOSDirective {
    fn from(directive: Directive) -> Self {
        match directive {
            Directive::Global(name) => MacOSDirective::Global(name),
            Directive::Extern(name) => MacOSDirective::Extern(name),
            Directive::Section(section_type) => MacOSDirective::Section(section_type.into()),
        }
    }
}

impl From<SectionType> for MacOSSectionType {
    fn from(section_type: SectionType) -> Self {
        match section_type {
            SectionType::Data => MacOSSectionType::Data,
            SectionType::Text => MacOSSectionType::Text,
            SectionType::Const => MacOSSectionType::Const,
            SectionType::Bss => MacOSSectionType::Bss,
        }
    }
}

impl From<Register> for MacOSRegister {
    fn from(register: Register) -> Self {
        match register {
            Register::Gp(x) => {
                match x {
                    1 => MacOSRegister::X1,
                    2 => MacOSRegister::X2,
                    3 => MacOSRegister::X3,
                    4 => MacOSRegister::X4,
                    5 => MacOSRegister::X5,
                    6 => MacOSRegister::X6,
                    7 => MacOSRegister::X7,
                    8 => MacOSRegister::X8,
                    9 => MacOSRegister::X9,
                    10 => MacOSRegister::X10,
                    11 => MacOSRegister::X11,
                    12 => MacOSRegister::X12,
                    13 => MacOSRegister::X13,
                    14 => MacOSRegister::X14,
                    15 => MacOSRegister::X15,
                    16 => MacOSRegister::X16,
                    17 => MacOSRegister::X17,
                    18 => MacOSRegister::X18,
                    19 => MacOSRegister::X19,
                    20 => MacOSRegister::X20,
                    21 => MacOSRegister::X21,
                    22 => MacOSRegister::X22,
                    23 => MacOSRegister::X23,
                    24 => MacOSRegister::X24,
                    25 => MacOSRegister::X25,
                    26 => MacOSRegister::X26,
                    27 => MacOSRegister::X27,
                    28 => MacOSRegister::X28,
                    29 => MacOSRegister::X29,
                    30 => MacOSRegister::X30,
                    31 => MacOSRegister::X31,
                    _ => panic!("Invalid general-purpose register: {}", x),
                }
            },
            Register::Fp(x) => {
                match x {
                    0 => MacOSRegister::V0,
                    1 => MacOSRegister::V1,
                    2 => MacOSRegister::V2,
                    3 => MacOSRegister::V3,
                    4 => MacOSRegister::V4,
                    5 => MacOSRegister::V5,
                    6 => MacOSRegister::V6,
                    7 => MacOSRegister::V7,
                    8 => MacOSRegister::V8,
                    9 => MacOSRegister::V9,
                    10 => MacOSRegister::V10,
                    11 => MacOSRegister::V11,
                    12 => MacOSRegister::V12,
                    13 => MacOSRegister::V13,
                    14 => MacOSRegister::V14,
                    15 => MacOSRegister::V15,
                    16 => MacOSRegister::V16,
                    17 => MacOSRegister::V17,
                    18 => MacOSRegister::V18,
                    19 => MacOSRegister::V19,
                    20 => MacOSRegister::V20,
                    21 => MacOSRegister::V21,
                    22 => MacOSRegister::V22,
                    23 => MacOSRegister::V23,
                    24 => MacOSRegister::V24,
                    25 => MacOSRegister::V25,
                    26 => MacOSRegister::V26,
                    27 => MacOSRegister::V27,
                    28 => MacOSRegister::V28,
                    29 => MacOSRegister::V29,
                    30 => MacOSRegister::V30,
                    31 => MacOSRegister::V31,
                    _ => panic!("Invalid floating-point register: {}", x),
                }
            },
            Register::Sp => MacOSRegister::Sp,
            Register::Pc => MacOSRegister::Pc,
            Register::Lr => MacOSRegister::Lr,
        }
    }
}

impl From<Op> for MacOSOp {
    fn from(op: Op) -> Self {
        match op {
            Op::Adrp => MacOSOp::Adrp,
            Op::Xor => MacOSOp::Xor,
            Op::Add => MacOSOp::Add,
            Op::Bl => MacOSOp::Bl,
            Op::Mov => MacOSOp::Mov,
            Op::Call => MacOSOp::Call,
            Op::Ret => MacOSOp::Ret,
            Op::Sub => MacOSOp::Sub,
            Op::Mul => MacOSOp::Mul,
            Op::Div => MacOSOp::Div,
            Op::And => MacOSOp::And,
            Op::Or => MacOSOp::Or,
        }
    }
}

impl From<Operand> for MacOSOperand {
    fn from(operand: Operand) -> Self {
        match operand {
            Operand::Register(reg) => MacOSOperand::Register(reg.into()),
            Operand::Immediate(val) => MacOSOperand::Immediate(val),
            Operand::Label(label) => MacOSOperand::Label(label),
            Operand::MemoryAddress(address) => MacOSOperand::MemoryAddress(address),
            Operand::MemoryOffset { base, offset } => MacOSOperand::MemoryOffset { base: base.into(), offset },
            Operand::ScaledIndex { base, index, scale } => MacOSOperand::ScaledIndex { base: base.into(), index: index.into(), scale },
        }
    }
}

impl Display for MacOSDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacOSDirective::Global(name) => write!(f, ".global {}", name),
            MacOSDirective::Extern(name) => write!(f, ".extern {}", name),
            MacOSDirective::Section(section) => write!(f, ".section {:?}", section),

        }
    }
}

impl Display for MacOSSectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacOSSectionType::Data => write!(f, ".data"),
            MacOSSectionType::Text => write!(f, ".text"),
            MacOSSectionType::Const => write!(f, ".const"),
            MacOSSectionType::Bss => write!(f, ".bss"),
        }
    }
}

impl Display for MacOSRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacOSRegister::X1 => write!(f, "x1"),
            MacOSRegister::X2 => write!(f, "x2"),
            MacOSRegister::X3 => write!(f, "x3"),
            MacOSRegister::X4 => write!(f, "x4"),
            MacOSRegister::X5 => write!(f, "x5"),
            MacOSRegister::X6 => write!(f, "x6"),
            MacOSRegister::X7 => write!(f, "x7"),
            MacOSRegister::X8 => write!(f, "x8"),
            MacOSRegister::X9 => write!(f, "x9"),
            MacOSRegister::X10 => write!(f, "x10"),
            MacOSRegister::X11 => write!(f, "x11"),
            MacOSRegister::X12 => write!(f, "x12"),
            MacOSRegister::X13 => write!(f, "x13"),
            MacOSRegister::X14 => write!(f, "x14"),
            MacOSRegister::X15 => write!(f, "x15"),
            MacOSRegister::X16 => write!(f, "x16"),
            MacOSRegister::X17 => write!(f, "x17"),
            MacOSRegister::X18 => write!(f, "x18"),
            MacOSRegister::X19 => write!(f, "x19"),
            MacOSRegister::X20 => write!(f, "x20"),
            MacOSRegister::X21 => write!(f, "x21"),
            MacOSRegister::X22 => write!(f, "x22"),
            MacOSRegister::X23 => write!(f, "x23"),
            MacOSRegister::X24 => write!(f, "x24"),
            MacOSRegister::X25 => write!(f, "x25"),
            MacOSRegister::X26 => write!(f, "x26"),
            MacOSRegister::X27 => write!(f, "x27"),
            MacOSRegister::X28 => write!(f, "x28"),
            MacOSRegister::X29 => write!(f, "x29"),
            MacOSRegister::X30 => write!(f, "x30"),
            MacOSRegister::X31 => write!(f, "x31"),
            MacOSRegister::V0 => write!(f, "v0"),
            MacOSRegister::V1 => write!(f, "v1"),
            MacOSRegister::V2 => write!(f, "v2"),
            MacOSRegister::V3 => write!(f, "v3"),
            MacOSRegister::V4 => write!(f, "v4"),
            MacOSRegister::V5 => write!(f, "v5"),
            MacOSRegister::V6 => write!(f, "v6"),
            MacOSRegister::V7 => write!(f, "v7"),
            MacOSRegister::V8 => write!(f, "v8"),
            MacOSRegister::V9 => write!(f, "v9"),
            MacOSRegister::V10 => write!(f, "v10"),
            MacOSRegister::V11 => write!(f, "v11"),
            MacOSRegister::V12 => write!(f, "v12"),
            MacOSRegister::V13 => write!(f, "v13"),
            MacOSRegister::V14 => write!(f, "v14"),
            MacOSRegister::V15 => write!(f, "v15"),
            MacOSRegister::V16 => write!(f, "v16"),
            MacOSRegister::V17 => write!(f, "v17"),
            MacOSRegister::V18 => write!(f, "v18"),
            MacOSRegister::V19 => write!(f, "v19"),
            MacOSRegister::V20 => write!(f, "v20"),
            MacOSRegister::V21 => write!(f, "v21"),
            MacOSRegister::V22 => write!(f, "v22"),
            MacOSRegister::V23 => write!(f, "v23"),
            MacOSRegister::V24 => write!(f, "v24"),
            MacOSRegister::V25 => write!(f, "v25"),
            MacOSRegister::V26 => write!(f, "v26"),
            MacOSRegister::V27 => write!(f, "v27"),
            MacOSRegister::V28 => write!(f, "v28"),
            MacOSRegister::V29 => write!(f, "v29"),
            MacOSRegister::V30 => write!(f, "v30"),
            MacOSRegister::V31 => write!(f, "v31"),
            MacOSRegister::W0 => write!(f, "w0"),
            MacOSRegister::W1 => write!(f, "w1"),
            MacOSRegister::W2 => write!(f, "w2"),
            MacOSRegister::W3 => write!(f, "w3"),
            MacOSRegister::W4 => write!(f, "w4"),
            MacOSRegister::W5 => write!(f, "w5"),
            MacOSRegister::W6 => write!(f, "w6"),
            MacOSRegister::W7 => write!(f, "w7"),
            MacOSRegister::W8 => write!(f, "w8"),
            MacOSRegister::W9 => write!(f, "w9"),
            MacOSRegister::W10 => write!(f, "w10"),
            MacOSRegister::W11 => write!(f, "w11"),
            MacOSRegister::W12 => write!(f, "w12"),
            MacOSRegister::W13 => write!(f, "w13"),
            MacOSRegister::W14 => write!(f, "w14"),
            MacOSRegister::W15 => write!(f, "w15"),
            MacOSRegister::W16 => write!(f, "w16"),
            MacOSRegister::W17 => write!(f, "w17"),
            MacOSRegister::W18 => write!(f, "w18"),
            MacOSRegister::W19 => write!(f, "w19"),
            MacOSRegister::W20 => write!(f, "w20"),
            MacOSRegister::W21 => write!(f, "w21"),
            MacOSRegister::W22 => write!(f, "w22"),
            MacOSRegister::W23 => write!(f, "w23"),
            MacOSRegister::W24 => write!(f, "w24"),
            MacOSRegister::W25 => write!(f, "w25"),
            MacOSRegister::W26 => write!(f, "w26"),
            MacOSRegister::W27 => write!(f, "w27"),
            MacOSRegister::W28 => write!(f, "w28"),
            MacOSRegister::W29 => write!(f, "w29"),
            MacOSRegister::W30 => write!(f, "w30"),
            MacOSRegister::W31 => write!(f, "w31"),
            MacOSRegister::Sp => write!(f, "sp"),
            MacOSRegister::Pc => write!(f, "pc"),
            MacOSRegister::Lr => write!(f, "lr"),
        }
    }
}

impl Display for MacOSOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacOSOp::Adrp => write!(f, "adrp"),
            MacOSOp::Add => write!(f, "add"),
            MacOSOp::Bl => write!(f, "bl"),
            MacOSOp::Mov => write!(f, "mov"),
            MacOSOp::Call => write!(f, "call"),
            MacOSOp::Ret => write!(f, "ret"),
            MacOSOp::Sub => write!(f, "sub"),
            MacOSOp::Mul => write!(f, "mul"),
            MacOSOp::Div => write!(f, "div"),
            MacOSOp::And => write!(f, "and"),
            MacOSOp::Or => write!(f, "or"),
            MacOSOp::Xor => write!(f, "xor"),
        }
    }
}

impl Display for MacOSOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacOSOperand::Register(reg) => write!(f, "{}", reg),
            MacOSOperand::Immediate(val) => write!(f, "#{}", val),
            MacOSOperand::Label(label) => write!(f, "{}", label),
            MacOSOperand::MemoryAddress(address) => write!(f, "[{}]", address),
            MacOSOperand::MemoryOffset { base, offset } => write!(f, "[{}, #{}]", base, offset),
            MacOSOperand::ScaledIndex { base, index, scale } => write!(f, "[{}, {}, lsl #{}]", base, index, scale),
        }
    }
}

impl Display for MacOSIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\t{}", self.op)?;
        for operand in &self.operands {
            write!(f, " {}", operand)?;
        }
        if let Some(comment) = &self.comment {
            write!(f, "\t// {}", comment)?;
        }
        Ok(())
    }
}


