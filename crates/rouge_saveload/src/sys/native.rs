use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use bevy_app::AppTypeRegistry;
use bevy_scene::DynamicScene;
use tracing::instrument;

#[instrument(skip(scene, type_registry))]
pub fn save_scene(filename: &str, scene: DynamicScene, type_registry: &AppTypeRegistry) {
    #[cfg(feature = "serialize-binary")]
    let serialized = super::super::serialize::serialize_bincode(scene, type_registry);
    #[cfg(not(feature = "serialize-binary"))]
    let serialized = super::super::serialize::serialize_ron(scene, type_registry);
    let compressed = super::super::compress::compress(&serialized);

    let path = Path::new(".").join(filename);
    let mut writer = File::create(path).unwrap();
    writer.write_all(&compressed).expect("Failed to save game!");
}

#[instrument(skip(type_registry))]
pub fn load_scene(filename: &str, type_registry: &AppTypeRegistry) -> Option<DynamicScene> {
    let path = Path::new(".").join(filename);
    let compressed = fs::read(path).expect("Failed to read save file!");
    let serialized = super::super::compress::decompress(&compressed);
    #[cfg(feature = "serialize-binary")]
    {
        Some(super::super::serialize::deserialize_bincode(
            &serialized,
            type_registry,
        ))
    }
    #[cfg(not(feature = "serialize-binary"))]
    {
        Some(super::super::serialize::deserialize_ron(
            &serialized,
            type_registry,
        ))
    }
}

#[instrument]
pub fn does_save_exist() -> bool {
    Path::new("./savegame.scn").exists()
}

#[instrument]
pub fn delete_save() {
    if Path::new("./savegame.scn").exists() {
        std::fs::remove_file("./savegame.scn").expect("Unable to delete file");
    }
}
