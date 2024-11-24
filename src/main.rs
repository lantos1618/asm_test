use std::collections::HashMap;
use assembler::*;
use serde_json;


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
    println!("{}", serde_json::to_string(&program).unwrap());
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
