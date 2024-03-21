//! Taken from https://github.com/tikv/minitrace-rust/blob/master/minitrace/examples/asynchronous.rs
//! Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

#![allow(clippy::new_without_default)]

use std::collections::HashMap;

use minitrace::collector::Config;
use minitrace::collector::ConsoleReporter;
use minitrace::collector::Reporter;
use minitrace::prelude::*;

fn parallel_job() -> Vec<tokio::task::JoinHandle<()>> {
    let mut v = Vec::with_capacity(4);
    for i in 0..4 {
        v.push(tokio::spawn(
            iter_job(i).in_span(Span::enter_with_local_parent("iter job")),
        ));
    }
    v
}

// #[trace(enter_on_poll = true)]
async fn iter_job(iter: u64) {
    std::thread::sleep(std::time::Duration::from_millis(iter * 10));
    tokio::task::yield_now().await;
    other_job().await;
}

#[trace(enter_on_poll = true)]
async fn other_job() {
    for i in 0..20 {
        if i == 10 {
            tokio::task::yield_now().await;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[tokio::main]
async fn main() {
    minitrace::set_reporter(MyReporter::new(), Config::default());
    // minitrace::set_reporter(ConsoleReporter, Config::default());

    {
        let parent = SpanContext::random();
        let span = Span::root("root", parent);

        let f = async {
            let jhs = {
                let _span = LocalSpan::enter_with_local_parent("a span")
                    .with_property(|| ("a property", "a value"));
                parallel_job()
            };

            other_job().await;

            for jh in jhs {
                jh.await.unwrap();
            }
        }
        .in_span(span);

        tokio::spawn(f).await.unwrap();
    }

    minitrace::flush();
}

struct MyReporter {
    map: HashMap<SpanId, String>,
}

impl MyReporter {
    fn new() -> Self {
        MyReporter {
            map: HashMap::new(),
        }
    }
}

impl Reporter for MyReporter {
    fn report(&mut self, spans: &[SpanRecord]) {
        for span in spans {
            self.map.insert(span.span_id, span.name.to_string());

            println!(
                "span: '{}'; parent: '{}'",
                span.name,
                self.map
                    .get(&span.parent_id)
                    .unwrap_or(&"no parent".to_string())
            );
        }
    }
}
