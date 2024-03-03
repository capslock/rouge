use tracing::level_filters::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};

/// Initialize tracing and configure subscribers.
pub fn init_tracing(name: String) {
    #[cfg(not(feature = "profile"))]
    {
        use tracing::Level;
        use tracing_subscriber::filter;

        let registry = tracing_subscriber::registry();

        let log_filter = filter::filter_fn(move |metadata| {
            *metadata.level() <= Level::WARN || metadata.target().starts_with(&name)
        });

        let layer = tracing_subscriber::fmt::layer().pretty().with_target(true);

        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env()
            .expect("failed to build env filter");

        registry.with(filter).with(log_filter).with(layer).init();
    }
    #[cfg(feature = "profile")]
    {
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
        )
        .expect("seting default subscriber failed");
    }
}
