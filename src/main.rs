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
                                        op: Op::Adrp { rd: Register::Gp(0), label: Operand::LabelWithModifier { label: "msg".to_string(), modifier: LabelModifier::Page } },
                                        comment: Comment::None,
                                    },
                                    Ins {
                                        op: Op::Add { rd: Register::Gp(0), rn: Register::Gp(0), op: Operand::LabelWithModifier { label: "msg".to_string(), modifier: LabelModifier::PageOff } },
                                        comment: Comment::None,
                                    },
                                    Ins {
                                        op: Op::Branch { label: Operand::Label("_printf".to_string()) },
                                        comment: Comment::None,
                                    },
                                    Ins {
                                        op: Op::Ret,
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
.section __DATA,__cstring
msg: .asciz "Hello, world!\n"
.section __TEXT,__text
_main:
    adrp x0, msg@l12
    add x0, x0, msg@l12
    bl _printf
    ret
"#;

        assert_eq!(expected, actual);
    }
}
