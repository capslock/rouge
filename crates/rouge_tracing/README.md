# rouge_tracing

Crate that sets up [tracing](https://crates.io/crates/tracing) with some
defaults that I found useful. Can be used with or without Bevy. If used with
Bevy, the default logging plugin should be disabled, e.g.:

```
App::new().add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
```

If you're using the full Bevy crate, you should probably just use the default
logging plugin and configure that to your liking.
