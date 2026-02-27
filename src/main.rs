use castellan::logging::prelude::*;
use castellan::settings::prelude::*;

use rig::{client::{CompletionClient, ProviderClient}, providers::gemini::Client};
use rig::completion::Prompt;
use tracing::{Level, event, span};
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let settings = settings();
    let _subscriber = logging_init(settings.app_log().level, settings.app_log().timestamp_mode);

    let _guard = span!(Level::INFO, "castellan_global").entered();
    event!(Level::INFO, "App start");

    if used_default_settings() {
        event!(Level::WARN, "Failed to parse configuration; using defaults")
    }

    let client = Client::from_env();

    let agent = client.agent("gemini-3-flash-preview")
        .preamble("You are a helpful assistant.")
        .name("Bob")
        .build();

    let prompt = "What is the Rust programming language?";
    println!("{prompt}");

    let response_text = agent.prompt(prompt).await?; 

    println!("Response: {response_text}");

    Ok(())

}
