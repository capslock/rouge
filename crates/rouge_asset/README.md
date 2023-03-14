# rouge_asset

THis is a crate that implements some additional Bevy asset plugins. Currently
this includes a plugin to load assets from the [dhall configuration
language](https://dhall-lang.org/).

Dhall asset files should end in `.dhall`. CBOR-encoded dhall files are also
supported and should have a `.dhallb` extension.

## Usage

```rust
#[derive(Deserialize, TypeUuid)]
#[uuid = "3eadc767-ffb8-45c7-9dc4-ae053725d290"]
struct GameData {}

fn main() {
    let mut app = App::new();
    app.add_plugin(DhallAssetPlugin::<GameData>::default());
    app.run();
}
```