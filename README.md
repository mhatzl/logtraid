# logtraid

Usage examples for log, tracing, and logid crates.

This repo contains examples to compare the three logging crates [log](https://github.com/rust-lang/log), [tracing](https://github.com/tokio-rs/tracing), and [logid](https://github.com/mhatzl/logid).
The examples are mostly taken from the `examples` folder in the tracing repo, and are slightly modified to get better comparison between the crates.

## Shaving Yaks Example

The sample code to *shave* yaks is used to showcase the different approach taken by `logid` compared to `log` and `tracing`.

- `log-yaks.rs` ... Contains the code using the `log` crate

  Sample output using `env_logger`:

  ```
  [2024-03-18T19:27:17Z INFO  log_yaks] preparing to shave yaks number_of_yaks=3
  [2024-03-18T19:27:17Z INFO  log_yaks] shaving yaks
  [2024-03-18T19:27:17Z TRACE log_yaks] hello! I'm gonna shave a yak excitement=yay!
  [2024-03-18T19:27:17Z TRACE log_yaks] yak shaved successfully
  [2024-03-18T19:27:17Z DEBUG yak_events]  yak=1 shaved=true
  [2024-03-18T19:27:17Z TRACE log_yaks]  yaks_shaved=1
  [2024-03-18T19:27:17Z TRACE log_yaks] hello! I'm gonna shave a yak excitement=yay!
  [2024-03-18T19:27:17Z TRACE log_yaks] yak shaved successfully
  [2024-03-18T19:27:17Z DEBUG yak_events]  yak=2 shaved=true
  [2024-03-18T19:27:17Z TRACE log_yaks]  yaks_shaved=2
  [2024-03-18T19:27:17Z TRACE log_yaks] hello! I'm gonna shave a yak excitement=yay!
  [2024-03-18T19:27:17Z WARN  log_yaks] could not locate yak
  [2024-03-18T19:27:17Z DEBUG yak_events]  yak=3 shaved=false
  [2024-03-18T19:27:17Z ERROR log_yaks] failed to shave yak yak=3 error=missing yak. cause: out of space. cause: out of cash
  [2024-03-18T19:27:17Z TRACE log_yaks]  yaks_shaved=2
  [2024-03-18T19:27:17Z INFO  log_yaks] yak shaving completed all_yaks_shaved=false
  ```

- `tracing-yaks.rs` ... Contains the code using the `tracing` crate

  Sample output using `tracing_subscriber`:

  ```
  2024-03-18T19:27:12.456667Z  INFO tracing_yaks: preparing to shave yaks number_of_yaks=3
  2024-03-18T19:27:12.457184Z  INFO shaving_yaks{yaks=3}: tracing_yaks: shaving yaks
  2024-03-18T19:27:12.457591Z TRACE shaving_yaks{yaks=3}:shave{yak=1}: tracing_yaks: hello! I'm gonna shave a yak excitement="yay!"
  2024-03-18T19:27:12.458006Z TRACE shaving_yaks{yaks=3}:shave{yak=1}: tracing_yaks: yak shaved successfully
  2024-03-18T19:27:12.458415Z DEBUG shaving_yaks{yaks=3}: yak_events: yak=1 shaved=true
  2024-03-18T19:27:12.458766Z TRACE shaving_yaks{yaks=3}: tracing_yaks: yaks_shaved=1
  2024-03-18T19:27:12.459107Z TRACE shaving_yaks{yaks=3}:shave{yak=2}: tracing_yaks: hello! I'm gonna shave a yak excitement="yay!"
  2024-03-18T19:27:12.459508Z TRACE shaving_yaks{yaks=3}:shave{yak=2}: tracing_yaks: yak shaved successfully
  2024-03-18T19:27:12.459849Z DEBUG shaving_yaks{yaks=3}: yak_events: yak=2 shaved=true
  2024-03-18T19:27:12.460184Z TRACE shaving_yaks{yaks=3}: tracing_yaks: yaks_shaved=2
  2024-03-18T19:27:12.460511Z TRACE shaving_yaks{yaks=3}:shave{yak=3}: tracing_yaks: hello! I'm gonna shave a yak excitement="yay!"
  2024-03-18T19:27:12.460874Z  WARN shaving_yaks{yaks=3}:shave{yak=3}: tracing_yaks: could not locate yak
  2024-03-18T19:27:12.461191Z DEBUG shaving_yaks{yaks=3}: yak_events: yak=3 shaved=false
  2024-03-18T19:27:12.461527Z ERROR shaving_yaks{yaks=3}: tracing_yaks: failed to shave yak yak=3 error="missing yak. cause: out of space. cause: out of cash"
  2024-03-18T19:27:12.461825Z TRACE shaving_yaks{yaks=3}: tracing_yaks: yaks_shaved=2
  2024-03-18T19:27:12.462082Z  INFO tracing_yaks: yak shaving completed all_yaks_shaved=false
  ```

- `logid-yaks.rs` ... Contains the code using the `logid` crate

  Sample output from `logid`:

  ```
  INFO  preparing to shave '3' yaks
  INFO  shaving yaks
  TRACE hello! I'm gonna shave a yak. yay!
  TRACE yak shaved successfully.
  TRACE shaved '1' yaks.
  TRACE hello! I'm gonna shave a yak. yay!
  TRACE yak shaved successfully.
  TRACE shaved '2' yaks.
  TRACE hello! I'm gonna shave a yak. yay!
  ERR   missing yak. cause: out of space. cause: out of cash
  ╰───> Info: Could not locate yak: 3
  TRACE shaved '2' yaks.
  INFO  not all yaks were shaved. Better luck next time.
  ```

## Tracing Spans

The `tracing` crate is able to create spans to help connect related logs during analysis.
Neither `log` nor `logid` are able to handle spans.

However, getting intended behavior for spans is non-trivial.
This is especially apparent in asynchronous projects, even though `tracing` is maintained by the Tokio team.
The two examples `tokio-spawny-thing.rs` and `spawny-thing.rs` are used to show one such scenario.

The big difference is using `tokio::spawn()` in `tokio-spawny-thing.rs` instead of awaiting the asynchronous function directly.

```rust
tokio::spawn(subtask(number).instrument(span))
```

vs

```rust
subtask(number)
```

This small difference already shows that it is not as easy as wrapping every function in `#[instrument]`.
Not using `#[instrument]` increases manual work to manage spans, but to get correct span nesting with `spawn()` calling `instrument(span)` is necessary.
This in turn will lead to the creation of two nearly identical spans.

## Structured Logging

`tracing` only allows to use primitive types, dyn Error, and Arguments for structured logging.
Meaning custom types cannot be directly logged.
`log` on the other hand added the `kv` feature to allow logging arbitrary types.

The sample `structured.rs` uses the `sval` crate to log a custom struct with `log`.
It also shows the current workaround for `tracing`.
