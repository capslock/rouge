use anyhow::Error;
use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt as _, BoxedFuture, LoadContext};
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
        app.register_asset_loader(DhallAssetLoader::<A>::default())
            .init_asset::<A>();
    }
}

struct DhallAssetLoader<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for DhallAssetLoader<A> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<A> AssetLoader for DhallAssetLoader<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    type Asset = A;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let ext = load_context.path().extension().unwrap();
            let asset = if ext.eq_ignore_ascii_case("dhall") {
                serde_dhall::from_str(std::str::from_utf8(&bytes)?)
                    .imports(false)
                    .parse::<Self::Asset>()?
            } else {
                serde_dhall::from_binary(&bytes)
                    .imports(false)
                    .parse::<Self::Asset>()?
            };
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dhall", "dhallb"]
    }
}
