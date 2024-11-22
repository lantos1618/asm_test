use std::fmt::Display;

pub trait Register: Display + Copy {}

pub trait ArithmeticBuilder<R: Register> {
    fn add(&mut self, dst: R, src1: R, src2: R);
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

#[derive(Debug)]
pub enum Condition {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    // ... etc
} 