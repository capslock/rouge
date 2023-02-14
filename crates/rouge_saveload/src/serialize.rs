use bevy_app::AppTypeRegistry;
use bevy_scene::serde::{SceneDeserializer, SceneSerializer};
use bevy_scene::DynamicScene;
use bincode::Options;
use serde::de::DeserializeSeed;

#[cfg(not(feature = "serialize-binary"))]
pub fn serialize(scene: DynamicScene, type_registry: &AppTypeRegistry) -> Vec<u8> {
    scene
        .serialize_ron(type_registry)
        .unwrap()
        .as_bytes()
        .to_owned()
}

#[cfg(not(feature = "serialize-binary"))]
pub fn deserialize(bytes: &[u8], type_registry: &AppTypeRegistry) -> DynamicScene {
    let scene_deserializer = SceneDeserializer {
        type_registry: &type_registry.0.write(),
    };
    let mut deserializer =
        ron::de::Deserializer::from_bytes(bytes).expect("Failed to create deserializer!");
    scene_deserializer
        .deserialize(&mut deserializer)
        .expect("Failed to deserialize scene!")
}

#[cfg(feature = "serialize-binary")]
pub fn serialize(scene: DynamicScene, type_registry: &AppTypeRegistry) -> Vec<u8> {
    let serializer = SceneSerializer::new(&scene, type_registry);
    bincode::serialize(&serializer).unwrap()
}

#[cfg(feature = "serialize-binary")]
pub fn deserialize(bytes: &[u8], type_registry: &AppTypeRegistry) -> DynamicScene {
    let scene_deserializer = SceneDeserializer {
        type_registry: &type_registry.0.write(),
    };

    let mut deserializer = bincode::Deserializer::from_slice(
        bytes,
        bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes(),
    );
    scene_deserializer
        .deserialize(&mut deserializer)
        .expect("Failed to deserialize scene!")
}
