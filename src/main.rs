use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Optional comment
type Comment = Option<String>;

/// Enums for assembly directives
#[derive(Debug, Serialize, Deserialize)]
enum Directive {
    Global(String),
    Extern(String),
    Section(SectionType),
}

/// Enum for section types
#[derive(Debug, Serialize, Deserialize)]
enum SectionType {
    Data,
    Text,
}

/// Enum for registers
#[derive(Debug, Serialize, Deserialize)]
enum Register {
    /// General-purpose register (e.g., x0-x30 for ARM64)
    Gp(u8),
    /// Floating-point register (e.g., f0-f31)
    Fp(u8),
    /// Special registers
    Sp, // Stack Pointer
    Pc, // Program Counter
    Lr, // Link Register
    // Add other special registers as needed
}

/// Enum for assembly operations
#[derive(Debug, Serialize, Deserialize)]
enum Op {
    Adrp,
    Add,
    Bl,
    Mov,
    Call,
    // ... other operations
}

/// Enum for operands in instructions
#[derive(Debug, Serialize, Deserialize)]
enum Operand {
    Register(Register),
    Immediate(i64),
    Label(String),
    MemoryAddress(String),
    // ... other operand types
}

/// Struct for assembly instructions
#[derive(Debug, Serialize, Deserialize)]
struct Ins {
    op: Op,
    operands: Vec<Operand>,
    comment: Comment,
}

/// Enum for label variants
#[derive(Debug, Serialize, Deserialize)]
enum LabelVariant {
    Data(DataLabel),
    Code(CodeLabel),
}

/// Struct for labels
#[derive(Debug, Serialize, Deserialize)]
struct Label {
    val: LabelVariant,
    comment: Comment,
}

/// Struct for code labels in code sections
#[derive(Debug, Serialize, Deserialize)]
struct CodeLabel {
    name: String,
    instructions: Vec<Ins>,
}

/// Struct for data labels in data sections
#[derive(Debug, Serialize, Deserialize)]
struct DataLabel {
    name: String,
    directive: String,
    value: String,
}

/// Struct for sections in the program
#[derive(Debug, Serialize, Deserialize)]
struct Section {
    section_type: SectionType,
    labels: Vec<Label>,
}

/// Struct for the entire assembly program
#[derive(Debug, Serialize, Deserialize)]
struct Program {
    directives: Vec<Directive>,
    sections: Vec<Section>,
}

fn main() {
    // Instantiate the Program struct with your assembly code
    let program = Program {
        directives: vec![
            Directive::Global("_start".to_string()),
            Directive::Extern("_printf".to_string()),
            Directive::Extern("_exit".to_string()),
        ],
        sections: vec![
            // Data section
            Section {
                section_type: SectionType::Data,
                labels: vec![Label {
                    val: LabelVariant::Data(DataLabel {
                        name: "msg".to_string(),
                        directive: ".asciz".to_string(),
                        value: "\"Result: %d\\n\"".to_string(),
                    }),
                    comment: Some("This is the message string".to_string()),
                }],
            },
            // Text section
            Section {
                section_type: SectionType::Text,
                labels: vec![Label {
                    val: LabelVariant::Code(CodeLabel {
                        name: "_start".to_string(),
                        instructions: vec![
                            Ins {
                                op: Op::Adrp,
                                operands: vec![
                                    Operand::Register(Register::Gp(0)), // x0
                                    Operand::Label("msg@PAGE".to_string()),
                                ],
                                comment: Some("Load page address of msg into x0".to_string()),
                            },
                            Ins {
                                op: Op::Add,
                                operands: vec![
                                    Operand::Register(Register::Gp(0)),
                                    Operand::Register(Register::Gp(0)),
                                    Operand::Label("msg@PAGEOFF".to_string()),
                                ],
                                comment: Some("Add offset of msg to x0".to_string()),
                            },
                            Ins {
                                op: Op::Bl,
                                operands: vec![Operand::Label("_printf".to_string())],
                                comment: Some("Call printf".to_string()),
                            },
                            Ins {
                                op: Op::Mov,
                                operands: vec![
                                    Operand::Register(Register::Gp(0)),
                                    Operand::Immediate(0),
                                ],
                                comment: Some("Set exit code to 0".to_string()),
                            },
                            Ins {
                                op: Op::Bl,
                                operands: vec![Operand::Label("_exit".to_string())],
                                comment: Some("Call exit".to_string()),
                            },
                        ],
                    }),
                    comment: Some("Entry point of the program".to_string()),
                }],
            },
        ],
    };

    // For demonstration, print the program structure
    println!("{:#?}", program);
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn hello_world_macos_arm64() {
       

//         let expected = r#".global _main
// .extern _printf
// .section __DATA,__data
// msg: .asciz "Hello, world!\n"
// .section __TEXT,__text
// _main:
//     adrp x0, msg@PAGE
//     add x0, x0, msg@PAGEOFF
//     bl _printf
//     ret
// "#;

//         // You need to implement a function to convert `program` to a string `actual`
//         // assert_eq!(expected, actual);
//     }
// }
