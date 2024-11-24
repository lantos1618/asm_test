use serde::{Serialize, Deserialize};
use serde_json;

/// Optional comment
type Comment = Option<String>;

/// Enums for assembly directives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directive {
    Global(String),
    Extern(String),
    Section(SectionType),
}

/// Enum for section types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionType {
    Data,
    Text,
    Const,
    Bss,
}

/// Enum for registers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Register {
    /// General-purpose register (e.g., x0-x30 for ARM64)
    Gp(u8),
    /// Floating-point register (e.g., f0-f31)
    Fp(u8),
    /// Special registers
    Sp, // Stack Pointer
    Pc, // Program Counter
    Lr, // Link Register

}

/// Enum for assembly operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
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

/// Enum for operands in instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
    Label(String),
    MemoryAddress(String),
    MemoryOffset { base: Register, offset: i64 },
    ScaledIndex { base: Register, index: Register, scale: u8 },
    // Add more operand types as needed
}

/// Struct for assembly instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ins {
    pub op: Op,
    pub operands: Vec<Operand>,
    pub comment: Comment,
}

/// Enum for label variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelVariant {
    Data(DataLabel),
    Code(CodeLabel),
}

/// Struct for labels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub val: LabelVariant,
    pub comment: Comment,
}

/// Struct for code labels in code sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLabel {
    pub name: String,
    pub instructions: Vec<Ins>,
}

/// Struct for data labels in data sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLabel {
    pub name: String,
    pub directive: String,
    pub value: String,
}

/// Struct for sections in the program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub section_type: SectionType,
    pub labels: Vec<Label>,
}

/// Struct for the entire assembly program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub sections: Vec<Section>,
}
