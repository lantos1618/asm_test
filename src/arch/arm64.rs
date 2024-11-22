use crate::instruction::*;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum Register {
    // General purpose registers
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15, X16, 
    // SIMD/FP registers
    V0, V1, V2, V3, V4, V5, V6, V7,
    // Special registers
    SP, LR, XZR,
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::X0 => write!(f, "x0"),
            Register::X1 => write!(f, "x1"),
            Register::X2 => write!(f, "x2"),
            // ... implement all registers
            Register::V0 => write!(f, "v0"),
            Register::V1 => write!(f, "v1"),
            Register::SP => write!(f, "sp"),
            Register::LR => write!(f, "lr"),
            Register::XZR => write!(f, "xzr"),
            _ => write!(f, "{:?}", self).map(|_| ()).map_err(|_| fmt::Error),
        }
    }
}

impl instruction::Register for Register {}

#[derive(Debug)]
pub enum Instruction {
    Arithmetic(ArithmeticOp),
    Branch(BranchOp),
    LoadStore(LoadStoreOp),
    System(SystemOp),
}

#[derive(Debug)]
pub enum ArithmeticOp {
    Add { dst: Register, src1: Register, src2: Register },
    Fadd { dst: Register, src1: Register, src2: Register },
    Sub { dst: Register, src1: Register, src2: Register },
    Mul { dst: Register, src1: Register, src2: Register },
}

#[derive(Debug)]
pub enum BranchOp {
    Bl { label: String },
    B { label: String },
    Ret,
    Cbz { reg: Register, label: String },
}

#[derive(Debug)]
pub enum LoadStoreOp {
    Ldr { dst: Register, src: String },
    Str { src: Register, dst: String },
}

#[derive(Debug)]
pub enum SystemOp {
    Svc { number: u32 },
    Msr { dst: String, src: Register },
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

impl ArithmeticBuilder<Register> for ARM64 {
    fn add(&mut self, dst: Register, src1: Register, src2: Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Add { dst, src1, src2 }
        ));
    }

    fn sub(&mut self, dst: Register, src1: Register, src2: Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Sub { dst, src1, src2 }
        ));
    }

    fn mul(&mut self, dst: Register, src1: Register, src2: Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Mul { dst, src1, src2 }
        ));
    }

    fn fadd(&mut self, dst: Register, src1: Register, src2: Register) {
        self.instructions.push(Instruction::Arithmetic(
            ArithmeticOp::Fadd { dst, src1, src2 }
        ));
    }
}

impl BranchBuilder<Register> for ARM64 {
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

    fn cbz(&mut self, reg: Register, label: &str) {
        self.instructions.push(Instruction::Branch(
            BranchOp::Cbz { reg, label: label.to_string() }
        ));
    }
} 