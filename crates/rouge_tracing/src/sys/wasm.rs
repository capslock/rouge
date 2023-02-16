use tracing::Level;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::{filter, prelude::*, FmtSubscriber};
use tracing_wasm::{WASMLayer, WASMLayerConfigBuilder};

pub fn init_tracing() {
    let config = WASMLayerConfigBuilder::default().build();
    let layer = WASMLayer::new(config);

    let filter = filter::filter_fn(|metadata| !metadata.target().starts_with("bevy"));
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .pretty()
        .with_target(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber.with(layer).with(filter))
        .expect("setting default subscriber failed");

    console_error_panic_hook::set_once();
}
