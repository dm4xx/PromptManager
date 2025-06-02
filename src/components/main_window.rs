use anyhow::Result;
use slint::{ComponentHandle, PlatformError};

slint::include_modules!();

pub struct MainWindowHandler {
    window: MainWindow,
}

impl MainWindowHandler {
    pub fn new() -> Result<Self> {
        let window = MainWindow::new()?;
        Ok(Self { window })
    }

    pub fn run(&self) -> Result<(), PlatformError> {
        self.window.run()
    }
} 