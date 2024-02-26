use bevy::ecs::reflect::AppTypeRegistry;
use bevy::scene::DynamicScene;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tracing::{error, instrument, warn};

lazy_static! {
    static ref SAVE_GAME: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

use crate::SaveloadError as Error;

/// Save a bevy `DynamicScene` to the file at the given `filename`.
#[instrument(skip(scene, type_registry))]
pub fn save_scene(
    filename: &str,
    scene: DynamicScene,
    type_registry: &AppTypeRegistry,
) -> Result<(), Error> {
    let serialized = super::super::serialize::serialize(scene, type_registry)?;
    let compressed = super::super::compress::compress(&serialized)?;
    let encoded = super::super::encode::encode(&compressed);

    if let Err(e) = web_sys::window().unwrap().local_storage() {
        warn!("Local storage unavailable: {:?}", e);
        *SAVE_GAME.lock() = compressed;
    } else if let Some(local_storage) = web_sys::window().unwrap().local_storage().unwrap() {
        let result = local_storage.set(filename, &encoded);
        if let Err(e) = result {
            error!("Failed to save game ({} bytes): {:?}", encoded.len(), e);
        }
    }
    Ok(())
}

/// Load a bevy `DynamicScene` from the file at the given `filename`.
#[instrument(skip(type_registry))]
pub fn load_scene(
    filename: &str,
    type_registry: &AppTypeRegistry,
) -> Result<Option<DynamicScene>, Error> {
    let compressed = if let Err(e) = web_sys::window().unwrap().local_storage() {
        warn!("Local storage unavailable: {:?}", e);
        std::mem::take(&mut *SAVE_GAME.lock())
    } else if let Some(local_storage) = web_sys::window().unwrap().local_storage().unwrap() {
        let encoded = local_storage.get(filename).unwrap().unwrap();
        super::super::encode::decode(&encoded)?
    } else {
        return Ok(None);
    };
    let serialized = super::super::compress::decompress(&compressed)?;

    Ok(Some(super::super::serialize::deserialize(
        &serialized,
        type_registry,
    )?))
}

/// Tests for the existence of a save file.
#[instrument]
pub fn does_save_exist() -> bool {
    if let Err(_e) = web_sys::window().unwrap().local_storage() {
        return !SAVE_GAME.lock().is_empty();
    }
    if let Some(local_storage) = web_sys::window().unwrap().local_storage().unwrap() {
        return local_storage.get_item("savegame.scn").unwrap().is_some();
    }
    false
}

/// Deletes a save file.
#[instrument]
pub fn delete_save() -> Result<(), Error> {
    if let Err(e) = web_sys::window().unwrap().local_storage() {
        warn!("Local storage unavailable: {:?}", e);
        std::mem::take(&mut *SAVE_GAME.lock());
        return Ok(());
    }
    if let Some(local_storage) = web_sys::window()
        .unwrap()
        .local_storage()
        .map_err(|message| Error::JS { message })?
    {
        local_storage.remove_item("savegame.scn").unwrap();
    }
    Ok(())
}
