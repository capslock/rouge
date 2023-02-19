use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use bevy_app::AppTypeRegistry;
use bevy_scene::DynamicScene;
use tracing::instrument;

use crate::SaveloadError as Error;

#[instrument(skip(scene, type_registry))]
pub fn save_scene(
    filename: &str,
    scene: DynamicScene,
    type_registry: &AppTypeRegistry,
) -> Result<(), Error> {
    let serialized = super::super::serialize::serialize(scene, type_registry)?;
    let compressed = super::super::compress::compress(&serialized)?;

    let path = Path::new(".").join(filename);
    let mut writer = File::create(path).map_err(Error::from)?;
    writer.write_all(&compressed).expect("Failed to save game!");
    Ok(())
}

#[instrument(skip(type_registry))]
pub fn load_scene(
    filename: &str,
    type_registry: &AppTypeRegistry,
) -> Result<Option<DynamicScene>, Error> {
    let path = Path::new(".").join(filename);
    let compressed = fs::read(path).map_err(Error::from)?;
    let serialized = super::super::compress::decompress(&compressed)?;
    Ok(Some(super::super::serialize::deserialize(
        &serialized,
        type_registry,
    )?))
}

#[instrument]
pub fn does_save_exist() -> bool {
    Path::new("./savegame.scn").exists()
}

#[instrument]
pub fn delete_save() -> Result<(), Error> {
    if Path::new("./savegame.scn").exists() {
        std::fs::remove_file("./savegame.scn").map_err(Error::from)?;
    }
    Ok(())
}
