use bevy::ecs::{entity::EntityMap, prelude::*};
use bevy::prelude::*;
use bevy::scene::DynamicScene;

use crate::{load_scene, save_scene};

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
        let scene = DynamicScene::from_world(world, type_registry);

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

        let mut entity_map = EntityMap::default();

        scene
            .write_to_world(world, &mut entity_map)
            .expect("Failed to write scene!");

        world.remove_resource::<SaveLoadState>();
    }
}

pub struct SaveloadPlugin {}

impl Plugin for SaveloadPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(save.in_base_set(CoreSet::PostUpdate));
        app.add_system(load.in_base_set(CoreSet::PreUpdate));
    }
}
