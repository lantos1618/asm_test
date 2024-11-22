use crate::instruction::*;

pub struct InstructionBuilder<A, R: Register> {
    pub arch: A,
    pub current_comment: Option<String>,
    _phantom: std::marker::PhantomData<R>,
}

impl<A, R: Register> InstructionBuilder<A, R> 
where
    GenericRegister: RegisterMapping<R>
{
    pub fn new(arch: A) -> Self {
        Self {
            arch,
            current_comment: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.current_comment = Some(comment.to_string());
        self
    }

    pub fn adrp(&mut self, dst: GenericRegister, label: &str) -> &mut Self 
    where 
        A: AddressBuilder<R>
    {
        self.arch.adrp(dst.to_arch_reg(), label);
        self
    }

    pub fn add(&mut self, dst: GenericRegister, src1: GenericRegister, src2: impl Into<Operand<R>>) -> &mut Self 
    where 
        A: ArithmeticBuilder<R>
    {
        let src2_op = match src2.into() {
            Operand::Register(r) => Operand::Register(r),
            Operand::Immediate(s) => Operand::Immediate(s),
        };
        
        self.arch.add(dst.to_arch_reg(), src1.to_arch_reg(), src2_op);
        self
    }

    pub fn bl(&mut self, label: &str) -> &mut Self 
    where 
        A: BranchBuilder<R>
    {
        self.arch.bl(label);
        self
    }

    pub fn mov(&mut self, dst: GenericRegister, src: GenericRegister) -> &mut Self 
    where 
        A: MovBuilder<R>
    {
        self.arch.mov(dst.to_arch_reg(), src.to_arch_reg());
        self
    }
} 