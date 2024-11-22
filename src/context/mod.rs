use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub variables: HashMap<String, Variable>,
    pub sections: Sections,
    pub label_counter: usize,
    pub externs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub label: String,
}

#[derive(Debug)]
pub struct Sections {
    text: Vec<(Option<String>, String)>,  // (comment, instruction)
    data: Vec<Variable>,
    bss: Vec<Variable>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            sections: Sections {
                text: Vec::new(),
                data: Vec::new(),
                bss: Vec::new(),
            },
            label_counter: 0,
            externs: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, name: &str, value: &str) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;

        let var = Variable {
            name: name.to_string(),
            value: value.to_string(),
            label: label.clone(),
        };

        self.sections.data.push(var.clone());
        self.variables.insert(name.to_string(), var);
        label
    }

    pub fn add_instruction(&mut self, instruction: String, comment: Option<String>) {
        self.sections.text.push((comment, instruction));
    }

    pub fn add_extern(&mut self, name: &str) {
        if !self.externs.contains(&name.to_string()) {
            self.externs.push(name.to_string());
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    pub fn get_sections(&self) -> &Sections {
        &self.sections
    }

    pub fn get_externs(&self) -> &[String] {
        &self.externs
    }
} 