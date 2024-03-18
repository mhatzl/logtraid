//! Example combined from
//! - https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt/yak_shave.rs
//! - https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-pretty.rs
//!
//! Removed `snafu` dependency that was used to create errors from enum variants.

use logid::{
    err,
    event_handler::builder::LogEventHandlerBuilder,
    log,
    log_id::LogLevel,
    logging::{
        event_entry::AddonKind,
        filter::{AddonFilter, FilterConfigBuilder},
    },
    ErrLogId, InfoLogId, TraceLogId,
};
use thiserror::Error;

pub fn shave(yak: usize) -> Result<(), YakError> {
    log!(ShavingTrace::Start);

    if yak == 3 {
        return err!(YakError::MissingYak {
            source: MissingYakError::OutOfSpace {
                source: OutOfSpaceError::OutOfCash,
            },
        }, add: AddonKind::Info(format!("Could not locate yak: {yak}")));
    }

    log!(ShavingTrace::Done);

    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    log!(ShavingInfo::ShavingYaks);

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(yak);

        if res.is_ok() {
            yaks_shaved += 1;
        }

        log!(ShavingTrace::Shaved(yaks_shaved));
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

#[derive(Debug, Error, ErrLogId)]
pub enum YakError {
    #[error("missing yak. cause: {}", .source)]
    MissingYak { source: MissingYakError },
}

#[derive(TraceLogId)]
pub enum ShavingTrace {
    Start,
    Done,
    Shaved(usize),
}

impl std::fmt::Display for ShavingTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShavingTrace::Start => write!(f, "hello! I'm gonna shave a yak. yay!"),
            ShavingTrace::Done => write!(f, "yak shaved successfully."),
            ShavingTrace::Shaved(nr) => write!(f, "shaved '{nr}' yaks."),
        }
    }
}

#[derive(InfoLogId)]
pub enum ShavingInfo {
    PrepShaving(usize),
    ShavingYaks,
    ShavingDone(bool),
}

impl std::fmt::Display for ShavingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShavingInfo::PrepShaving(nr) => write!(f, "preparing to shave '{nr}' yaks"),
            ShavingInfo::ShavingYaks => write!(f, "shaving yaks"),
            ShavingInfo::ShavingDone(all_shaved) => {
                if *all_shaved {
                    write!(f, "yak shaving completed successfully!")
                } else {
                    write!(f, "not all yaks were shaved. Better luck next time.")
                }
            }
        }
    }
}

fn main() {
    let _ = logid::logging::filter::set_filter(
        FilterConfigBuilder::new(LogLevel::Trace)
            .allowed_addons([AddonFilter::Infos])
            .build(),
    );

    let _handler = LogEventHandlerBuilder::new()
        .to_stderr()
        .all_log_events()
        .build()
        .unwrap();

    let number_of_yaks = 3;

    log!(ShavingInfo::PrepShaving(number_of_yaks));

    let number_shaved = shave_all(number_of_yaks);

    log!(ShavingInfo::ShavingDone(number_shaved == number_of_yaks));

    // Makes sure to print all outstanding logs
    // NOTE: Possible fix might be to require calling `handler.join()`, or moving to `log` as *backend*.
    std::thread::sleep(std::time::Duration::from_millis(10));
}
