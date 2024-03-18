use futures::future::try_join_all;
use tracing::{debug, info, instrument, span, Instrument as _, Level};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[instrument]
async fn parent_task(subtasks: usize) -> Result<(), Error> {
    info!("spawning subtasks...");

    let subtasks = (1..=subtasks)
        .map(|number| {
            let span = span!(Level::INFO, "subtask-spawn", %number);

            debug!(message = "creating subtask;", number);

            // NOTE: Removing `.instrument(span)` here results in `parent_task` **not** being associated with `subtask`.
            tokio::spawn(subtask(number).instrument(span))
        })
        .collect::<Vec<_>>();

    // the returnable error would be if one of the subtasks panicked.
    let sum: usize = try_join_all(subtasks).await?.iter().sum();

    info!(%sum, "all subtasks completed; calculated sum");

    Ok(())
}

// NOTE: Removing `#[instrument]` here would prevent creating span `subtask` on top of `subtask-spawn`,
// but then calling `subtask` directly would not create any span at all.
#[instrument]
async fn subtask(number: usize) -> usize {
    info!(%number, "polling subtask");

    number
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init()?;

    parent_task(10).await?;
    Ok(())
}
