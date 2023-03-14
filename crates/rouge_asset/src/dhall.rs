use anyhow::Error;
use bevy::app::{App, Plugin};
use bevy::asset::{AddAsset, Asset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use std::marker::PhantomData;

/// Plugin that loads assets of type `A` from a dhall file.
pub struct DhallAssetPlugin<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for DhallAssetPlugin<A> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<A> Plugin for DhallAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.add_asset::<A>()
            .add_asset_loader(DhallAssetLoader::<A> {
                _marker: PhantomData,
            });
    }
}

struct DhallAssetLoader<A> {
    _marker: PhantomData<A>,
}

impl<A> AssetLoader for DhallAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let ext = load_context.path().extension().unwrap();
            let asset = if ext.eq_ignore_ascii_case("dhall") {
                serde_dhall::from_str(std::str::from_utf8(bytes)?)
                    .imports(false)
                    .parse::<A>()?
            } else {
                serde_dhall::from_binary(bytes)
                    .imports(false)
                    .parse::<A>()?
            };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dhall", "dhallb"]
    }
}
