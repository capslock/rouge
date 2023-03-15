# rouge

rouge is a collection of roguelike utility crates developed when working through
the Rust [Roguelike Tutorial](https://bfnightly.bracketproductions.com/) and
building [Exalted](https://left-pad.itch.io/exalted-7drl).

## Crates

### rouge

Meta-crate that includes all of the other sub-crates.

#### Features

* `default`: enables the `asset`, `bevy`, `gui`, `queue`, `saveload`,
  `serialize_binary`, `tracing` features.
* `asset`: enables the `rouge_asset` crate.
* `bevy`: enables the `bevy` feature for `rouge_gui`.
* `gui`: enables the `rouge_gui` crate.
* `queue`: enables the `rouge_queue` crate
* `saveload`: enables the `rouge_saveload` crate.
* `serialize-binary`: enables the `serialize-binary` feature for
  `rouge_saveload`.
* `tracing`: enables the `rouge_tracing` crate.

### rouge_asset

Crate that adds some additional Bevy asset plugins. Currently this includes a
plugin to load assets from the [dhall configuration
language](https://dhall-lang.org/).

### rouge_gui

Crate that implements an immediate-mode style text-mode gui useable with
bracket-lib or bracket-bevy. Very rough and very incomplete, but I've gotten
some good mileage out of it already. It can simplify some of the boilerplate of
creating windows and handling input.

#### Features

* `bevy`: Makes the crate compatible with `bracket-bevy`.

### rouge_queue

Crate that adds a `queue` resource type for use with Bevy. Can be used as an
n-to-1 communication channel between systems, supporting both concurrent readers
and writers without needing exclusive access to the resource. Built on
[crossbeam](https://crates.io/crates/crossbeam)'s `SegQueue`. Can be used with
full Bevy via the plugin, or with just the `bevy_ecs` crate.

### rouge_saveload

`wasm32-none-none`-compatible save/load system built with Bevy in mind. Probably
needs some work to adapt it to particular use-cases, but it does handle several
corner-cases of `wasm` saveload handling such that I found it useful to pull out
into a separate crate. Can be used with full Bevy or just `bevy_ecs`.

Can serialize into either a text format (via Bevy's inbuilt support for
[ron](https://crates.io/crates/ron)) or binary (via
[bincode](https://crates.io/crates/bincode)).

#### Features

* `default`: Enables the `serialize-binary` feature.
* `serialize-binary`: Use binary save-file serialization via
  [bincode](https://crates.io/crates/bincode).

### rouge_tracing

Crate that sets up [tracing](https://crates.io/crates/tracing) with some
defaults that I found useful. Can be used with or without Bevy. If used with
Bevy, the default logging plugin should be disabled.

If you're using the full Bevy crate, you should probably just use the default
logging plugin and configure that to your liking.

#### Features

* `profile`: Enable profiling support with
  [tracing-tracy](https://crates.io/crates/tracing-tracy).