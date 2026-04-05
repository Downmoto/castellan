use std::env;

use rig::prelude::CompletionClient;
use rig::{completion::Prompt, providers::openai};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum LlmError {
    #[error("missing OPENAI_API_KEY")]
    MissingApiKey,
    #[error("llm request failed: {0}")]
    Request(String),
    #[error("empty llm response")]
    EmptyResponse,
}

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