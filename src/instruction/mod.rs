use std::fmt::Display;

pub trait Register: Display + Copy {
    fn is_general_purpose(&self) -> bool;
    fn is_floating_point(&self) -> bool;
    fn is_special(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GenericRegister {
    // General purpose registers (X0-X30)
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    // SIMD/FP registers (V0-V31)
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
    // Special registers
    SP,  // Stack Pointer
    LR,  // Link Register (alias for X30)
    XZR, // Zero Register
}

impl GenericRegister {
    pub fn validate_for_arm64(&self) -> Result<(), &'static str> {
        match self {
            Self::X0
            | Self::X1
            | Self::X2
            | Self::X3
            | Self::X4
            | Self::X5
            | Self::X6
            | Self::X7
            | Self::X8
            | Self::X9
            | Self::X10
            | Self::X11
            | Self::X12
            | Self::X13
            | Self::X14
            | Self::X15
            | Self::X16
            | Self::X17
            | Self::X18
            | Self::X19
            | Self::X20
            | Self::X21
            | Self::X22
            | Self::X23
            | Self::X24
            | Self::X25
            | Self::X26
            | Self::X27
            | Self::X28
            | Self::X29
            | Self::X30
            | Self::SP
            | Self::LR
            | Self::XZR => Ok(()),
            Self::SP | Self::LR | Self::XZR => Ok(()),
            _ => Err("Invalid register for ARM64 architecture"),
        }
    }
}

pub trait RegisterMapping<R: Register> {
    fn to_arch_reg(&self) -> R;
}

pub enum Operand<R> {
    Register(R),
    Immediate(String),
}

pub trait ArithmeticBuilder<R: Register> {
    fn add(&mut self, dst: R, src1: R, src2: Operand<R>);
    fn sub(&mut self, dst: R, src1: R, src2: R);
    fn mul(&mut self, dst: R, src1: R, src2: R);
    fn fadd(&mut self, dst: R, src1: R, src2: R);
}

pub trait BranchBuilder<R: Register> {
    fn bl(&mut self, label: &str);
    fn b(&mut self, label: &str);
    fn ret(&mut self);
    fn cbz(&mut self, reg: R, label: &str);
}

pub trait LoadStoreBuilder<R: Register> {
    fn str(&mut self, src: R, addr: &str);
    fn ldr(&mut self, dst: R, addr: &str);
}

pub trait MovBuilder<R: Register> {
    fn mov(&mut self, dst: R, src: R);
}

pub trait AddressBuilder<R: Register> {
    fn adrp(&mut self, dst: R, label: &str);
    fn adrp_add(&mut self, dst: R, base: R, label: &str);
}
