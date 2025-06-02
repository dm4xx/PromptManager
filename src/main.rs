use anyhow::Result;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

mod app;
mod components;
mod database;
mod models;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize logging
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
		.init();

	// Initialize the application
	let app = app::PromptManagerApp::new()?;
	app.run().await?;
	Ok(())
}
