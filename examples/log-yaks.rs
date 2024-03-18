//! Example combined from
//! - https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt/yak_shave.rs
//! - https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-pretty.rs
//!
//! Removed `snafu` dependency that was used to create errors from enum variants.

use log::{debug, error, info, trace};
use thiserror::Error;

pub fn shave(yak: usize) -> Result<(), YakError> {
    trace!(excitement = "yay!"; "hello! I'm gonna shave a yak");

    if yak == 3 {
        log::warn!("could not locate yak");

        return Err(YakError::MissingYak {
            source: MissingYakError::OutOfSpace {
                source: OutOfSpaceError::OutOfCash,
            },
        });
    } else {
        trace!("yak shaved successfully");
    }

    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    info!("shaving yaks");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(yak);

        debug!(target: "yak_events", yak, shaved = res.is_ok(); "");

        if let Err(ref error) = res {
            error!(yak, error = error.to_string().as_str(); "failed to shave yak");
        } else {
            yaks_shaved += 1;
        }

        trace!(yaks_shaved; "");
    }

    yaks_shaved
}

#[derive(Debug, Error)]
pub enum OutOfSpaceError {
    #[error("out of cash")]
    OutOfCash,
}

#[derive(Debug, Error)]
pub enum MissingYakError {
    #[error("out of space. cause: {}", .source)]
    OutOfSpace { source: OutOfSpaceError },
}

#[derive(Debug, Error)]
pub enum YakError {
    #[error("missing yak. cause: {}", .source)]
    MissingYak { source: MissingYakError },
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let number_of_yaks = 3;

    info!(number_of_yaks; "preparing to shave yaks");

    let number_shaved = shave_all(number_of_yaks);

    info!(
        all_yaks_shaved = number_shaved == number_of_yaks;
        "yak shaving completed"
    );
}
