use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub prompt_text: String,
    pub ai_service: String,
    pub created_at: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptTree {
    pub root_id: String,
    pub nodes: Vec<PromptNode>,
}
