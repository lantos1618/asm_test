pub mod arch;
pub mod platform;
pub mod instruction;

// Re-export commonly used items
pub use arch::arm64::{ARM64, Register as ARM64Register};
pub use platform::macos::MacOS;
pub use instruction::{ArithmeticBuilder, BranchBuilder, Register};

// The main program struct
use std::marker::PhantomData;

pub struct Program<A, R: Register, P> {
    pub arch: A,
    platform: P,
    _phantom: PhantomData<R>,
}

impl<A, R: Register, P: platform::Platform> Program<A, R, P> 
where 
    A: ArithmeticBuilder<R> + BranchBuilder<R>
{
    pub fn new(arch: A, platform: P) -> Self {
        Self {
            arch,
            platform,
            _phantom: PhantomData,
        }
    }

    pub fn add_float_numbers(&mut self, dst: R, src1: R, src2: R) {
        self.arch.fadd(dst, src1, src2);
    }
} 