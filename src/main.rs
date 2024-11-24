use std::collections::HashMap;
use assembler::*;
use serde_json;


fn main() {
   
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_macos_arm64() {
        
        let program = Program {
            directives: vec![
                Directive::Global("_main".to_string()),
                Directive::Extern("_printf".to_string()),
            ], 
            sections: vec![
                Section {
                    section_type: SectionType::Data,
                    labels: vec![
                        Label {
                            val: LabelVariant::Data(DataLabel {
                                name: "msg".to_string(),
                                directive: "asciz".to_string(),
                                value: "Hello, world!\n".to_string(),
                            }),
                            comment: Comment::None,
                        }
                    ],
                },
                Section {
                    section_type: SectionType::Text,
                    labels: vec![
                        Label {
                            val: LabelVariant::Code(CodeLabel {
                                name: "_main".to_string(),
                                instructions: vec![
                                   Ins {
                                    op: Op::Adrp { rd: Register::Gp(0), label: "msg".to_string() },
                                    comment: Comment::None,
                                   },
                                   Ins {
                                    op: Op::Add { rd: Register::Gp(0), rn: Register::Gp(0), rm: Register::Gp(0) },
                                    comment: Comment::None,
                                   },
                                   Ins {
                                    op: Op::Bl { label: "_printf".to_string() },
                                    comment: Comment::None,
                                   },
                                ],
                            }),
                            comment: Comment::None,
                        }
                    ],
                }
            ],
        };

        let macos_program: MacOSProgram = program.into();
        let actual = macos_program.to_string();
        let expected = r#".global _main
.extern _printf
.section __DATA,__data
msg: .asciz "Hello, world!\n"
.section __TEXT,__text
_main:
    adrp x0, msg@PAGE
    add x0, x0, msg@PAGEOFF
    bl _printf
    ret
"#;

        // You need to implement a function to convert `program` to a string `actual`
        assert_eq!(expected, actual);
    }
}
