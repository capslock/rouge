use anyhow::{anyhow, Error};
use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetApp, AssetLoader, AsyncReadExt as _, BoxedFuture, LoadContext};
use nickel_lang_core::eval::cache::lazy::CBNCache;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// Plugin that loads assets of type `A` from a Nickel file.
pub struct NickelAssetPlugin<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for NickelAssetPlugin<A> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<A> Plugin for NickelAssetPlugin<A>
where
    for<'de> A: serde::Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.register_asset_loader(NickelAssetLoader::<A>::default())
            .init_asset::<A>();
    }
}

struct NickelAssetLoader<A> {
    _marker: PhantomData<A>,
}

impl<A> Default for NickelAssetLoader<A> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<A> AssetLoader for NickelAssetLoader<A>
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
            let bytes = VecDeque::from(bytes);
            let writer = Vec::new();
            let mut program = nickel_lang_core::program::Program::<CBNCache>::new_from_source(
                bytes,
                load_context.asset_path().to_string(),
                writer,
            )?;
            let rich_term = program
                .eval_full_for_export()
                .map_err(|e| anyhow!("failed to eval: {:?}", e))?;
            let asset = Self::Asset::deserialize(rich_term)?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ncl"]
    }
}
