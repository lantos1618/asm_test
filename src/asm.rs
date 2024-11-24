use serde::{Serialize, Deserialize};
use serde_json;

/// Optional comment
pub type Comment = Option<String>;

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
    Gp(u8),      // General-purpose register (e.g., x0-x30 on ARM64 or r0-r15 on x86)
    Fp(u8),      // Floating-point register
    Sp,          // Stack pointer
    Pc,          // Program counter
    Lr,          // Link register (e.g., x30 on ARM64)
    // Add architecture-specific registers as needed
}

/// Enum for assembly operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
    // Data movement
    Mov { rd: Register, src: Operand },          // Generalized move operation
    Load { rd: Register, addr: Operand },       // Load from memory
    Store { rs: Register, addr: Operand },      // Store to memory

    // Arithmetic operations
    Add { rd: Register, rn: Register, op: Operand },
    Sub { rd: Register, rn: Register, op: Operand },
    Mul { rd: Register, rn: Register, op: Operand },
    Div { rd: Register, rn: Register, op: Operand },

    // Logical operations
    And { rd: Register, rn: Register, op: Operand },
    Or { rd: Register, rn: Register, op: Operand },
    Xor { rd: Register, rn: Register, op: Operand },
    Not { rd: Register, rn: Register },

    // Bitwise operations
    Shl { rd: Register, rn: Register, amount: u8 }, // Shift left
    Shr { rd: Register, rn: Register, amount: u8 }, // Logical shift right
    Sar { rd: Register, rn: Register, amount: u8 }, // Arithmetic shift right

    // Address computation
    Lea { rd: Register, addr: Operand },           // Load effective address
    Adrp { rd: Register, label: Operand },         // ARM64-specific address of page

    // Branching and control flow
    Call { label: Operand },                        // Call a function
    Ret,                                           // Return
    Branch { label: Operand },                      // Unconditional branch
    BranchIf { cond: Condition, label: Operand },   // Conditional branch

    // SIMD/Vector operations
    VecAdd { rd: Register, rn: Register, rm: Register }, // Vectorized addition
    VecMul { rd: Register, rn: Register, rm: Register }, // Vectorized multiplication

    // Atomic/Threading
    AtomicLoad { rd: Register, addr: Operand },    // Atomic load
    AtomicStore { rs: Register, addr: Operand },   // Atomic store
    AtomicAdd { rd: Register, rn: Register, addr: Operand }, // Atomic addition

    // System-specific
    SysCall { num: u64 },       // System call with number
    Nop,                        // No operation
    Halt,                       // Halt the system
}

/// Enum for operands in instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    Register(Register),                    // A CPU register
    Immediate(i64),                        // Immediate value
    MemoryAddress(String),                 // Direct memory address (label or symbol)
    MemoryOffset { base: Register, offset: i64 }, // Address with offset
    ScaledIndex { base: Register, index: Register, scale: u8 }, // Base + index * scale
    LabelWithModifier { label: String, modifier: LabelModifier }, // Label with modifiers like :lo12:, @PAGE
    Label(String), // Label without modifiers
}

/// Struct for assembly instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ins {
    pub op: Op,
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

/// Enum for label variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Eq,  // Equal
    Ne,  // Not equal
    Lt,  // Less than
    Gt,  // Greater than
    Le,  // Less than or equal
    Ge,  // Greater than or equal
    // Add architecture-specific flags (e.g., Zero, Overflow)
}

/// Enum for label modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelModifier {
    Page,      // @PAGE
    PageOff,   // @PAGEOFF
    Lo12,      // :lo12:
    Hi20,      // :hi20:
    // Add other modifiers as needed
}
