pub mod arm64;
pub mod x86_64;

use std::fmt::Debug;

// Add these new type definitions
#[derive(Debug)]
pub struct Program {
    pub directives: Vec<Directive>,
    pub sections: Vec<Section>,
}

impl Program {
    pub fn section(&mut self, section: Section) -> &mut Section {
        self.sections.push(section);
        self.sections.last_mut().unwrap()
    }

    pub fn directive(&mut self, directive: Directive) {
        self.directives.push(directive);
    }
}

#[derive(Debug)]
pub enum Directive {
    Global(String),
    Extern(String),
}

#[derive(Debug)]
pub enum Section {
    Data(DataSection),
    Text(TextSection),
}

impl Section {
    pub fn add_string(&mut self, label: String, value: String) -> &StringData {
        match self {
            Section::Data(data_section) => data_section.add_string(label, value),
            Section::Text(_) => panic!("Text sections do not support strings"),
        }
    }

    pub fn add_block(&mut self, name: String) -> &Block {
        match self {
            Section::Text(text_section) => text_section.add_block(name),
            Section::Data(_) => panic!("Data sections do not support blocks"),
        }
    }


}

#[derive(Debug)]
pub struct DataSection {
    pub strings: Vec<StringData>,
}

impl DataSection {
    pub fn add_string(&mut self, label: String, value: String) -> &StringData {
        let string = StringData { label, value };
        self.strings.push(string);
        self.strings.last().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct StringData {
    pub label: String,
    pub value: String,
}



impl StringData {
    pub fn page(&self) -> String {
        format!("{}@PAGE", self.label)
    }

    pub fn page_off(&self) -> String {
        format!("{}@PAGEOFF", self.label)
    }
}

#[derive(Debug)]
pub struct TextSection {
    pub blocks: Vec<Block>,
}

impl TextSection {
    pub fn add_block(&mut self, name: String) -> &Block {
        let block = Block { name, instructions: Vec::new() };
        self.blocks.push(block);
        self.blocks.last().unwrap()
    }
}

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub instructions: Vec<Ins>,
}

impl Block {
    pub fn ins(&mut self, instruction: Ins) {
        self.instructions.push(instruction);
    }
}

#[derive(Debug)]
pub enum Ins {
    Adrp { dest: String, symbol: String },
    Add { dest: String, src: String, symbol: String },
    Mov { dest: String, src: Operand },
    Bl { symbol: String },
    Call { symbol: String },
    Ret,
    Nop,
}

#[derive(Debug)]
pub enum Operand {
    Register(String),
    Immediate(i64),
    Symbol(String),
}

// Core traits
pub trait Target: Debug {
    fn compile_directive(&self, directive: &Directive) -> String;
    fn compile_instruction(&self, instruction: &Ins) -> String;
    fn section_data_directive(&self) -> String;
    fn section_text_directive(&self) -> String;
}

pub trait Register: Debug + Clone {
    fn name(&self) -> &'static str;
}

// Main compiler struct
#[derive(Debug)]
pub struct Compiler<T: Target> {
    target: T,
}

impl<T: Target> Compiler<T> {
    pub fn new(target: T) -> Self {
        Self { target }
    }

    pub fn compile(&self, program: &Program) -> String {
        let mut output = String::new();
        
        // Compile directives
        for directive in &program.directives {
            output.push_str(&self.target.compile_directive(directive));
            output.push('\n');
        }

        // Compile sections
        for section in &program.sections {
            match section {
                Section::Data(data_section) => {
                    output.push_str(&format!("\n{}\n", self.target.section_data_directive()));
                    for string in &data_section.strings {
                        output.push_str(&format!("{}:.asciz \"{}\"\n", 
                            string.label, string.value));
                    }
                }
                Section::Text(text_section) => {
                    output.push_str(&format!("\n{}\n", self.target.section_text_directive()));
                    for block in &text_section.blocks {
                        output.push_str(&format!("{}:\n", block.name));
                        for instruction in &block.instructions {
                            output.push_str("    ");
                            output.push_str(&self.target.compile_instruction(instruction));
                            output.push('\n');
                        }
                    }
                }
            }
        }
        output
    }
}