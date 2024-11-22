use crate::instruction::*;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arm64Register {
    X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, 
    X11, X12, X13, X14, X15, X16, X17, X18, X19, X20,
    X21, X22, X23, X24, X25, X26, X27, X28, X29, X30,
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10,
    V11, V12, V13, V14, V15, V16, V17, V18, V19, V20,
    V21, V22, V23, V24, V25, V26, V27, V28, V29, V30,
    V31,
    SP, LR, XZR,
}

impl Register for Arm64Register {
    fn is_general_purpose(&self) -> bool {
        matches!(self, 
            Self::X0 | Self::X1 | Self::X2 | Self::X3 |
            Self::X4 | Self::X5 | Self::X6 | Self::X7 |
            Self::X8 | Self::X9 | Self::X10 | Self::X11 |
            Self::X12 | Self::X13 | Self::X14 | Self::X15
        )
    }

    fn is_floating_point(&self) -> bool {
        matches!(self,
            Self::V0 | Self::V1 | Self::V2 | Self::V3 |
            Self::V4 | Self::V5 | Self::V6 | Self::V7
        )
    }

    fn is_special(&self) -> bool {
        matches!(self,
            Self::SP | Self::LR | Self::XZR
        )
    }
}

impl Display for Arm64Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X0 => write!(f, "x0"),
            Self::X1 => write!(f, "x1"),
            Self::X2 => write!(f, "x2"),
            Self::X3 => write!(f, "x3"),
            Self::X4 => write!(f, "x4"),
            Self::X5 => write!(f, "x5"),
            Self::X6 => write!(f, "x6"),
            Self::X7 => write!(f, "x7"),
            Self::X8 => write!(f, "x8"),
            Self::X9 => write!(f, "x9"),
            Self::X10 => write!(f, "x10"),
            Self::X11 => write!(f, "x11"),
            Self::X12 => write!(f, "x12"),
            Self::X13 => write!(f, "x13"),
            Self::X14 => write!(f, "x14"),
            Self::X15 => write!(f, "x15"),
            Self::X16 => write!(f, "x16"),
            Self::X17 => write!(f, "x17"),
            Self::X18 => write!(f, "x18"),
            Self::X19 => write!(f, "x19"),
            Self::X20 => write!(f, "x20"),
            Self::X21 => write!(f, "x21"),
            Self::X22 => write!(f, "x22"),
            Self::X23 => write!(f, "x23"),
            Self::X24 => write!(f, "x24"),
            Self::X25 => write!(f, "x25"),
            Self::X26 => write!(f, "x26"),
            Self::X27 => write!(f, "x27"),
            Self::X28 => write!(f, "x28"),
            Self::X29 => write!(f, "x29"),
            Self::X30 => write!(f, "x30"),
            Self::V0 => write!(f, "v0"),
            Self::V1 => write!(f, "v1"),
            Self::V2 => write!(f, "v2"),
            Self::V3 => write!(f, "v3"),
            Self::V4 => write!(f, "v4"),
            Self::V5 => write!(f, "v5"),
            Self::V6 => write!(f, "v6"),
            Self::V7 => write!(f, "v7"),
            Self::V8 => write!(f, "v8"),
            Self::V9 => write!(f, "v9"),
            Self::V10 => write!(f, "v10"),
            Self::V11 => write!(f, "v11"),
            Self::V12 => write!(f, "v12"),
            Self::V13 => write!(f, "v13"),
            Self::V14 => write!(f, "v14"),
            Self::V15 => write!(f, "v15"),
            Self::V16 => write!(f, "v16"),
            Self::V17 => write!(f, "v17"),
            Self::V18 => write!(f, "v18"),
            Self::V19 => write!(f, "v19"),
            Self::V20 => write!(f, "v20"),
            Self::V21 => write!(f, "v21"),
            Self::V22 => write!(f, "v22"),
            Self::V23 => write!(f, "v23"),
            Self::V24 => write!(f, "v24"),
            Self::V25 => write!(f, "v25"),
            Self::V26 => write!(f, "v26"),
            Self::V27 => write!(f, "v27"),
            Self::V28 => write!(f, "v28"),
            Self::V29 => write!(f, "v29"),
            Self::V30 => write!(f, "v30"),
            Self::V31 => write!(f, "v31"),
            Self::SP => write!(f, "sp"),
            Self::LR => write!(f, "lr"),
            Self::XZR => write!(f, "xzr"),
        }
    }
}

impl From<GenericRegister> for Arm64Register {
    fn from(reg: GenericRegister) -> Self {
        match reg {
            GenericRegister::X0 => Arm64Register::X0,
            GenericRegister::X1 => Arm64Register::X1,
            GenericRegister::X2 => Arm64Register::X2,
            GenericRegister::X3 => Arm64Register::X3,
            GenericRegister::X4 => Arm64Register::X4,
            GenericRegister::X5 => Arm64Register::X5,
            GenericRegister::X6 => Arm64Register::X6,
            GenericRegister::X7 => Arm64Register::X7,
            GenericRegister::X8 => Arm64Register::X8,
            GenericRegister::X9 => Arm64Register::X9,
            GenericRegister::X10 => Arm64Register::X10,
            GenericRegister::X11 => Arm64Register::X11,
            GenericRegister::X12 => Arm64Register::X12,
            GenericRegister::X13 => Arm64Register::X13,
            GenericRegister::X14 => Arm64Register::X14,
            GenericRegister::X15 => Arm64Register::X15,
            GenericRegister::X16 => Arm64Register::X16,
            GenericRegister::X17 => Arm64Register::X17,
            GenericRegister::X18 => Arm64Register::X18,
            GenericRegister::X19 => Arm64Register::X19,
            GenericRegister::X20 => Arm64Register::X20,
            GenericRegister::X21 => Arm64Register::X21,
            GenericRegister::X22 => Arm64Register::X22,
            GenericRegister::X23 => Arm64Register::X23,
            GenericRegister::X24 => Arm64Register::X24,
            GenericRegister::X25 => Arm64Register::X25,
            GenericRegister::X26 => Arm64Register::X26,
            GenericRegister::X27 => Arm64Register::X27,
            GenericRegister::X28 => Arm64Register::X28,
            GenericRegister::X29 => Arm64Register::X29,
            GenericRegister::X30 => Arm64Register::X30,
            GenericRegister::V0 => Arm64Register::V0,
            GenericRegister::V1 => Arm64Register::V1,
            GenericRegister::V2 => Arm64Register::V2,
            GenericRegister::V3 => Arm64Register::V3,
            GenericRegister::V4 => Arm64Register::V4,
            GenericRegister::V5 => Arm64Register::V5,
            GenericRegister::V6 => Arm64Register::V6,
            GenericRegister::V7 => Arm64Register::V7,
            GenericRegister::V8 => Arm64Register::V8,
            GenericRegister::V9 => Arm64Register::V9,
            GenericRegister::V10 => Arm64Register::V10,
            GenericRegister::V11 => Arm64Register::V11,
            GenericRegister::V12 => Arm64Register::V12,
            GenericRegister::V13 => Arm64Register::V13,
            GenericRegister::V14 => Arm64Register::V14,
            GenericRegister::V15 => Arm64Register::V15,
            GenericRegister::V16 => Arm64Register::V16,
            GenericRegister::V17 => Arm64Register::V17,
            GenericRegister::V18 => Arm64Register::V18,
            GenericRegister::V19 => Arm64Register::V19,
            GenericRegister::V20 => Arm64Register::V20,
            GenericRegister::V21 => Arm64Register::V21,
            GenericRegister::V22 => Arm64Register::V22,
            GenericRegister::V23 => Arm64Register::V23,
            GenericRegister::V24 => Arm64Register::V24,
            GenericRegister::V25 => Arm64Register::V25,
            GenericRegister::V26 => Arm64Register::V26,
            GenericRegister::V27 => Arm64Register::V27,
            GenericRegister::V28 => Arm64Register::V28,
            GenericRegister::V29 => Arm64Register::V29,
            GenericRegister::V30 => Arm64Register::V30,
            GenericRegister::V31 => Arm64Register::V31,
            GenericRegister::SP => Arm64Register::SP,
            GenericRegister::LR => Arm64Register::LR,
            GenericRegister::XZR => Arm64Register::XZR,
            _ => panic!("Invalid register {:?} for ARM64 architecture", reg),
        }
    }
}

impl RegisterMapping<Arm64Register> for GenericRegister {
    fn to_arch_reg(&self) -> Arm64Register {
        Arm64Register::from(*self)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Arithmetic(ArithmeticOp),
    Branch(BranchOp),
    LoadStore(LoadStoreOp),
    System(SystemOp),
    Address(AddressOp),
}

#[derive(Debug)]
pub enum ArithmeticOp {
    Add { dst: Arm64Register, src1: Arm64Register, src2: Arm64Register },
    AddImm { dst: Arm64Register, src1: Arm64Register, imm: String },
    Fadd { dst: Arm64Register, src1: Arm64Register, src2: Arm64Register },
    Sub { dst: Arm64Register, src1: Arm64Register, src2: Arm64Register },
    Mul { dst: Arm64Register, src1: Arm64Register, src2: Arm64Register },
}

#[derive(Debug)]
pub enum BranchOp {
    Bl { label: String },
    B { label: String },
    Ret,
    Cbz { reg: Arm64Register, label: String },
}

#[derive(Debug)]
pub enum LoadStoreOp {
    Ldr { dst: Arm64Register, src: String },
    Str { src: Arm64Register, dst: String },
}

#[derive(Debug)]
pub enum SystemOp {
    Svc { number: u32 },
    Msr { dst: String, src: Arm64Register },
}

#[derive(Debug)]
pub enum AddressOp {
    Adrp { dst: Arm64Register, label: String },
    AdrpAdd { dst: Arm64Register, base: Arm64Register, label: String },
}

pub struct ARM64 {
    instructions: Vec<Instruction>,
}

impl ARM64 {
    pub fn new() -> Self {
        Self { instructions: Vec::new() }
    }

    pub fn get_instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}

impl ARM64 {
    pub fn add(&mut self, dst: GenericRegister, src1: GenericRegister, src2: GenericRegister) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Add { 
                dst: dst.to_arch_reg(), 
                src1: src1.to_arch_reg(), 
                src2: src2.to_arch_reg() 
            }
        ));
    }
}

impl BranchBuilder<Arm64Register> for ARM64 {
    fn bl(&mut self, label: &str) {
        self.instructions.push(Instruction::Branch(
            BranchOp::Bl { label: label.to_string() }
        ));
    }

    fn b(&mut self, label: &str) {
        self.instructions.push(Instruction::Branch(
            BranchOp::B { label: label.to_string() }
        ));
    }

    fn ret(&mut self) {
        self.instructions.push(Instruction::Branch(BranchOp::Ret));
    }

    fn cbz(&mut self, reg: Arm64Register, label: &str) {
        self.instructions.push(Instruction::Branch(
            BranchOp::Cbz { reg, label: label.to_string() }
        ));
    }
}

impl LoadStoreBuilder<Arm64Register> for ARM64 {
    fn str(&mut self, src: Arm64Register, addr: &str) {
        self.instructions.push(Instruction::LoadStore(
            LoadStoreOp::Str { src, dst: addr.to_string() }
        ));
    }

    fn ldr(&mut self, dst: Arm64Register, addr: &str) {
        self.instructions.push(Instruction::LoadStore(
            LoadStoreOp::Ldr { dst, src: addr.to_string() }
        ));
    }
}

impl MovBuilder<Arm64Register> for ARM64 {
    fn mov(&mut self, dst: Arm64Register, src: Arm64Register) {
        // For ARM64, mov is actually an alias for orr with XZR
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Add { dst, src1: src, src2: Arm64Register::XZR }
        ));
    }
}

impl AddressBuilder<Arm64Register> for ARM64 {
    fn adrp(&mut self, dst: Arm64Register, label: &str) {
        self.instructions.push(Instruction::Address(
            AddressOp::Adrp { 
                dst, 
                label: label.to_string() 
            }
        ));
    }

    fn adrp_add(&mut self, dst: Arm64Register, base: Arm64Register, label: &str) {
        self.instructions.push(Instruction::Address(
            AddressOp::AdrpAdd { 
                dst, 
                base,
                label: label.to_string() 
            }
        ));
    }
}

impl ArithmeticBuilder<Arm64Register> for ARM64 {
    fn add(&mut self, dst: Arm64Register, src1: Arm64Register, src2: Operand<Arm64Register>) {
        match src2 {
            Operand::Register(reg) => {
                self.instructions.push(Instruction::Arithmetic(
                    ArithmeticOp::Add { dst, src1, src2: reg }
                ));
            },
            Operand::Immediate(imm) => {
                self.instructions.push(Instruction::Arithmetic(
                    ArithmeticOp::AddImm { dst, src1, imm }
                ));
            }
        }
    }

    fn sub(&mut self, dst: Arm64Register, src1: Arm64Register, src2: Arm64Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Sub { dst, src1, src2 }
        ));
    }

    fn mul(&mut self, dst: Arm64Register, src1: Arm64Register, src2: Arm64Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Mul { dst, src1, src2 }
        ));
    }

    fn fadd(&mut self, dst: Arm64Register, src1: Arm64Register, src2: Arm64Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Fadd { dst, src1, src2 }
        ));
    }
}

// Add conversion from String to Operand
impl From<String> for Operand<Arm64Register> {
    fn from(s: String) -> Self {
        Operand::Immediate(s)
    }
}

impl<'a> From<&'a str> for Operand<Arm64Register> {
    fn from(s: &'a str) -> Self {
        Operand::Immediate(s.to_string())
    }
}

// Add this implementation
impl From<GenericRegister> for Operand<Arm64Register> {
    fn from(reg: GenericRegister) -> Self {
        Operand::Register(reg.to_arch_reg())
    }
} 