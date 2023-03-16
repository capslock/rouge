use bevy::app::AppTypeRegistry;
use bevy::scene::serde::SceneDeserializer;
#[cfg(feature = "serialize-binary")]
use bevy::scene::serde::SceneSerializer;
use bevy::scene::DynamicScene;
#[cfg(feature = "serialize-binary")]
use bincode::Options;
use serde::de::DeserializeSeed;

use crate::SaveloadError as Error;

/// Serialize a bevy `DynamicScene` to a `Vec<u8>`.
#[cfg(not(feature = "serialize-binary"))]
pub fn serialize(scene: DynamicScene, type_registry: &AppTypeRegistry) -> Vec<u8> {
    scene
        .serialize_ron(type_registry)
        .unwrap()
        .as_bytes()
        .to_owned()
}

/// Deserialize a bevy `DynamicScene` from a byte slice.
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

/// Serialize a bevy `DynamicScene` to a `Vec<u8>`.
#[cfg(feature = "serialize-binary")]
pub fn serialize(scene: DynamicScene, type_registry: &AppTypeRegistry) -> Result<Vec<u8>, Error> {
    let serializer = SceneSerializer::new(&scene, type_registry);
    bincode::serialize(&serializer).map_err(|source| Error::Serialize(source.into()))
}

/// Deserialize a bevy `DynamicScene` from a byte slice.
#[cfg(feature = "serialize-binary")]
pub fn deserialize(bytes: &[u8], type_registry: &AppTypeRegistry) -> Result<DynamicScene, Error> {
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
        .map_err(|source| Error::Deserialize(source.into()))
}
