use tracing_subscriber::prelude::*;

pub fn init_tracing() {
    #[cfg(not(feature = "profile"))]
    {
        use tracing::Level;
        use tracing_subscriber::{filter, FmtSubscriber};

        let filter = filter::filter_fn(|metadata| !metadata.target().starts_with("bevy"));
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .pretty()
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber.with(filter))
            .expect("setting default subscriber failed");
    }
    #[cfg(feature = "profile")]
    {
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()),
        )
        .expect("seting default subscriber failed");
    }
}
