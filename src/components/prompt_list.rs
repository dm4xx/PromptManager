use slint::ModelRc;

pub struct PromptListModel {
    items: Vec<String>,
}

impl PromptListModel {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_prompt(&mut self, prompt: String) {
        self.items.push(prompt);
    }
}
