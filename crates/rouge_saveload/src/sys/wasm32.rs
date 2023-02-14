use bevy_app::AppTypeRegistry;
use bevy_scene::DynamicScene;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tracing::{error, instrument, warn};

lazy_static! {
    static ref SAVE_GAME: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

#[instrument(skip(scene, type_registry))]
pub fn save_scene(filename: &str, scene: DynamicScene, type_registry: &AppTypeRegistry) {
    let serialized = super::super::serialize::serialize_ron(scene, type_registry);
    let compressed = super::super::compress::compress(&serialized);
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
}

#[instrument(skip(type_registry))]
pub fn load_scene(filename: &str, type_registry: &AppTypeRegistry) -> Option<DynamicScene> {
    let compressed = if let Err(e) = web_sys::window().unwrap().local_storage() {
        warn!("Local storage unavailable: {:?}", e);
        std::mem::take(&mut *SAVE_GAME.lock())
    } else if let Some(local_storage) = web_sys::window().unwrap().local_storage().unwrap() {
        let encoded = local_storage.get(filename).unwrap().unwrap();
        super::super::encode::decode(&encoded)
    } else {
        return None;
    };
    let serialized = super::super::compress::decompress(&compressed);

    Some(super::super::serialize::deserialize_ron(
        &serialized,
        type_registry,
    ))
}

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

#[instrument]
pub fn delete_save() {
    if let Err(e) = web_sys::window().unwrap().local_storage() {
        warn!("Local storage unavailable: {:?}", e);
        std::mem::take(&mut *SAVE_GAME.lock());
        return;
    }
    if let Some(local_storage) = web_sys::window().unwrap().local_storage().unwrap() {
        local_storage.remove_item("savegame.scn").unwrap();
    }
}
