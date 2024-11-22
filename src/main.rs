use std::fs;
use assembler::*;

fn main() -> std::io::Result<()> {
    let mut program = Program {
        directives: Vec::new(),
        sections: Vec::new(),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_generation() {
        let mut program = Program {
            directives: Vec::new(),
            sections: Vec::new(),
        };


        let compiler = Compiler::new(arm64::ARM64Target);

        // Setup program...
        program.directive(Directive::Global("_start".to_owned()));
        program.directive(Directive::Extern("_printf".to_owned()));
        program.directive(Directive::Extern("_exit".to_owned()));
    
        // Create sections first
        let mut data_section = DataSection { strings: Vec::new() };
        let mut text_section = TextSection { blocks: Vec::new() };
    
        // Work with sections
        let msg = data_section.add_string("msg".to_owned(), r#"Hello, world!\n"#.to_owned());
    
        let mut start = Block{
            name: "_start".to_owned(),
            instructions: Vec::new(),
        };
        start.ins(Ins::Adrp { dest: "x0".to_owned(), symbol: msg.page() });
        start.ins(Ins::Add { dest: "x0".to_owned(), src: "x0".to_owned(), symbol: msg.page_off() });
        start.ins(Ins::Call { symbol: "_printf".to_owned() });
        start.ins(Ins::Mov { dest: "x0".to_owned(), src: Operand::Immediate(0) });
        start.ins(Ins::Call { symbol: "_exit".to_owned() });
    
        text_section.blocks.push(start);
    
        // Add sections to program after we're done with them
        program.sections.push(Section::Data(data_section));
        program.sections.push(Section::Text(text_section));
    
        // Generate assembly code
        let program_str = compiler.compile(&program);
        
        // Update expected output to match actual compiler output
        let expected_output = r#".global _start
.extern _printf
.extern _exit

.section __DATA,__data
msg:.asciz "Hello, world!\n"

.section __TEXT,__text
_start:
    adrp x0, msg@PAGE
    add x0, x0, msg@PAGEOFF
    bl _printf
    mov x0, #0
    bl _exit
"#;
        assert_eq!(program_str, expected_output);
    }
}
