use futures::future::join_all;
use std::error::Error;
use tracing::{debug, info, instrument};

#[instrument]
async fn parent_task(subtasks: usize) {
    info!("spawning subtasks...");

    let subtasks = (1..=subtasks)
        .map(|number| {
            debug!(message = "creating subtask;", number);

            subtask(number)
        })
        .collect::<Vec<_>>();

    let result = join_all(subtasks).await;

    debug!("all subtasks completed");

    let sum: usize = result.into_iter().sum();

    info!(%sum, "all subtasks completed; calculated sum");
}

#[instrument]
async fn subtask(number: usize) -> usize {
    info!(%number, "polling subtask");

    number
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;

    parent_task(10).await;
    Ok(())
}
