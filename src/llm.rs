//! llm request execution and assistant reply transport.
//!
//! this module owns api key/model lookup, prompt execution, and the async task
//! handoff that sends replies back to the ui event loop.

use std::env;

use rig::prelude::CompletionClient;
use rig::{completion::Prompt, providers::openai};
use thiserror::Error;
use tokio::sync::mpsc;


/// errors surfaced when generating an assistant response.
#[derive(Debug, Error)]
pub enum LlmError {
    /// the required `OPENAI_API_KEY` variable is not present.
    #[error("missing OPENAI_API_KEY")]
    MissingApiKey,
    /// the provider client or request returned an error message.
    #[error("llm request failed: {0}")]
    Request(String),
    /// the provider returned only whitespace content.
    #[error("empty llm response")]
    EmptyResponse,
}

/// reply payload routed from background llm tasks back to the app loop.
pub struct AssistantReply {
    /// destination tab index at submit time.
    pub tab_index: usize,
    /// assistant text to append to the tab transcript.
    pub message: String,
}

/// stateless launcher for background llm reply jobs.
#[derive(Clone, Copy, Debug, Default)]
pub struct LlmService;

impl LlmService {
    /// creates a new stateless llm service handle.
    pub fn new() -> Self {
        Self
    }

    /// starts an async task that generates and sends an assistant reply.
    ///
    /// this method does not block the caller. failures are converted into a
    /// user-facing error message string and still sent over `sender` so the ui
    /// can surface feedback in the transcript.
    pub fn request_reply(
        self,
        sender: mpsc::Sender<AssistantReply>,
        tab_index: usize,
        user_prompt: String,
    ) {
        tokio::spawn(async move {
            let message = assistant_message_from_prompt(&user_prompt).await;
            let _ = sender.send(AssistantReply { tab_index, message }).await;
        });
    }
}

fn format_user_facing_error(error: LlmError) -> String {
    format!(
        "llm request failed: {error}. set OPENAI_API_KEY and optional CAST_LLM_MODEL."
    )
}

async fn assistant_message_from_prompt(user_prompt: &str) -> String {
    match generate_reply(user_prompt).await {
        Ok(content) => content,
        Err(error) => format_user_facing_error(error),
    }
}

/// generates a single assistant response for the provided user prompt.
///
/// environment variables:
/// - `OPENAI_API_KEY` (required): authentication key.
/// - `CAST_LLM_MODEL` (optional): model id override.
///
/// returns trimmed response text or a typed `LlmError`.
pub async fn generate_reply(user_prompt: &str) -> Result<String, LlmError> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| LlmError::MissingApiKey)?;
    let model = env::var("CAST_LLM_MODEL").unwrap_or_else(|_| openai::GPT_4O_MINI.to_string());

    let client: openai::Client =
        openai::Client::new(&api_key).map_err(|error| LlmError::Request(error.to_string()))?;

    let agent = client
        .agent(model)
        .preamble("you are a concise assistant in a terminal ui. keep answers short and useful.")
        .build();

    let content = agent
        .prompt(user_prompt)
        .await
        .map_err(|error| LlmError::Request(error.to_string()))?
        .trim()
        .to_string();

    if content.is_empty() {
        return Err(LlmError::EmptyResponse);
    }

    Ok(content)
}