use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use bevy::scene::DynamicScene;

use crate::{load_scene, save_scene};

/// Flag resource that indicates that the game should be saved.
///
/// To save the game, insert a `SaveLoadState` resource with the value
/// [`SaveLoadState::Save`]. To load a save, insert the value
/// [`SaveLoadState::Load`].
#[derive(Resource)]
pub enum SaveLoadState {
    Save,
    Load,
}

fn save(world: &mut World) {
    if matches!(
        world.get_resource::<SaveLoadState>(),
        Some(SaveLoadState::Save)
    ) {
        let type_registry = world.resource::<AppTypeRegistry>();
        let scene = DynamicScene::from_world(world);

        save_scene("savegame.scn", scene, type_registry).expect("Failed to save game!");

        world.remove_resource::<SaveLoadState>();
    }
}

fn load(world: &mut World) {
    if matches!(
        world.get_resource::<SaveLoadState>(),
        Some(SaveLoadState::Load)
    ) {
        world.clear_entities();
        world.clear_trackers();

        let scene = {
            let type_registry = world.get_resource::<AppTypeRegistry>().unwrap();
            if let Some(scene) =
                load_scene("savegame.scn", type_registry).expect("Failed to load game!")
            {
                scene
            } else {
                return;
            }
        };

        let mut entity_map = EntityHashMap::default();

        scene
            .write_to_world(world, &mut entity_map)
            .expect("Failed to write scene!");

        world.remove_resource::<SaveLoadState>();
    }
}

/// Plugin that implements a save/load system.
///
/// To save the game, insert a [`SaveLoadState`] resource with the value
/// [`SaveLoadState::Save`]. To load a save, insert the value
/// [`SaveLoadState::Load`].
#[derive(Default)]
pub struct SaveloadPlugin {}

impl Plugin for SaveloadPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(PostUpdate, save);
        app.add_systems(PreUpdate, load);
    }
}
