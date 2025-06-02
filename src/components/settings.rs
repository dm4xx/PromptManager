pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            theme: "light".to_string(),
            font_size: 12,
        }
    }
}
