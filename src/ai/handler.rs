use anyhow::Result;

use crate::{app::AppState, jobs::Job, models::message::Message};

pub struct StreamResult {
    pub msg_id: String,
    pub content: String,
    pub reasoning: Option<String>,
}

pub fn create_title_prompt(msg: &str) -> String {
    format!(
        "Summarize the following message into a short, concise title of 5 words or less, without quotation marks: \"{}\"",
        msg
    )
}

pub fn enqueue_ai_jobs(
    state: &AppState,
    user_id: String,
    chat_id: String,
    new_msg_body: String,
    messages: Vec<Message>,
) -> Result<()> {
    if messages.len() == 1 {
        state.job_tx.send(Job::GenerateTitle {
            chat_id: chat_id.clone(),
            user_id: user_id.clone(),
            first_body: new_msg_body,
        })?;
    }

    state.job_tx.send(Job::GenerateResponse {
        chat_id,
        user_id,
        messages: messages,
    })?;

    Ok(())
}
