use anyhow::Result;
use tracing::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::database::PromptDatabase;
use std::path::PathBuf;
use crate::components::main_window::MainWindowHandler;

pub struct PromptManagerApp {
    window: MainWindowHandler,
    database: Arc<Mutex<PromptDatabase>>,
}

impl PromptManagerApp {
    pub fn new() -> Result<Self> {
        let database = Arc::new(Mutex::new(PromptDatabase::new(PathBuf::from("prompt_data"))?));
        let window = MainWindowHandler::new()?;
        Ok(Self {
            window,
            database,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting PromptManager application");
        self.window.run()?;
        Ok(())
    }
}
