# rouge_saveload

`wasm32-none-none`-compatible save/load system built with Bevy in mind. Probably
needs some work to adapt it to particular use-cases, but it does handle several
corner-cases of `wasm` saveload handling such that I found it useful to pull out
into a separate crate. Can be used with full Bevy or just `bevy_ecs`.

Can serialize into either a text format (via Bevy's inbuilt support for
[ron](https://crates.io/crates/ron)) or binary (via
[bincode](https://crates.io/crates/bincode)).

## Features

* `default`: Enables the `serialize-binary` feature.
* `serialize-binary`: Use binary save-file serialization via
  [bincode](https://crates.io/crates/bincode).