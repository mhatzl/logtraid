#[derive(sval_derive::Value)]
pub struct CustomType {
    v1: &'static str,
    v2: u32,
    v3: bool,
}

pub fn main() {
    if rand::random() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    } else {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Trace)
            .init();
    }

    let c = CustomType {
        v1: "Value 1",
        v2: 2,
        v3: true,
    };

    log::info!(c:sval; "Logging custom structs works with `log` using `sval` or `serde`."); // see: https://docs.rs/log/latest/log/kv/index.html

    // tracing::info!(
    //     c,
    //     "This breaks, because `tracing` sealed the `Value` trait."
    // ); // see: https://docs.rs/tracing/latest/tracing/trait.Value.html

    // alternative:
    tracing::info!(c.v1, c.v2, c.v3, "Every field must be set manually.");
}
