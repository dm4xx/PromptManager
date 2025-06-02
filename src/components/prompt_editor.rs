use slint::ModelRc;

pub struct PromptEditor {
    current_prompt: String,
}

impl PromptEditor {
    pub fn new() -> Self {
        Self { current_prompt: String::new() }
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.current_prompt = prompt;
    }
}
