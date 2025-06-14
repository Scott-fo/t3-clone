use anyhow::Result;
use tokio::sync::mpsc;
use tokio_retry2::{
    Retry, RetryError,
    strategy::{ExponentialBackoff, jitter},
};

use crate::{
    ai::handler::{generate_response, generate_title},
    app::AppState,
    models::message::Message,
};

#[derive(Debug, Clone)]
pub enum Job {
    GenerateTitle {
        chat_id: String,
        user_id: String,
        first_body: String,
    },
    GenerateResponse {
        chat_id: String,
        user_id: String,
        messages: Vec<Message>,
    },
}

pub async fn run_worker(state: AppState, mut rx: mpsc::UnboundedReceiver<Job>) {
    while let Some(job) = rx.recv().await {
        let state_cloned = state.clone();

        tokio::spawn(async move {
            let strategy = ExponentialBackoff::from_millis(500).map(jitter).take(3);

            let result = Retry::spawn(strategy, || {
                let state_inner = state_cloned.clone();
                let job_inner = job.clone();
                async move {
                    handle_job(&state_inner, job_inner)
                        .await
                        .map_err(RetryError::transient)
                }
            })
            .await;

            if let Err(e) = result {
                tracing::error!(error = ?e, job = ?job, "Job permanently failed");
            }
        });
    }
}

async fn handle_job(state: &AppState, job: Job) -> Result<()> {
    match job {
        Job::GenerateTitle {
            chat_id,
            user_id,
            first_body,
        } => generate_title(state, chat_id, user_id, first_body).await?,

        Job::GenerateResponse {
            chat_id,
            user_id,
            messages,
        } => generate_response(state, chat_id, user_id, messages).await?,
    }
    Ok(())
}
