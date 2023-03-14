use bevy::{app::Plugin, prelude::World};

use crate::Queue;

type QueueFn = dyn Fn(&mut bevy::app::App) + Send + Sync;

/// Plugin that automatically adds [`Queue`] resources to the app with the types
/// provided to the [`QueuePlugin::with_queue`] builder.
///
/// This plugin can be initialized multiple times, so it can be safely used from
/// other plugins in the same application.
#[derive(Default)]
pub struct QueuePlugin {
    types: Vec<Box<QueueFn>>,
}

impl QueuePlugin {
    /// Add a [`Queue`] of type `T` to the application.
    pub fn with_queue<T: Send + 'static>(mut self) -> Self {
        self.types.push(Box::new(|app: &mut bevy::app::App| {
            app.init_resource::<Queue<T>>();
        }));
        self
    }
}

impl Plugin for QueuePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        for t in self.types.iter() {
            t(app);
        }
    }

    fn is_unique(&self) -> bool {
        false
    }
}

/// Trait for types that can add a [`Queue`] resource.
pub trait AddQueue {
    /// Add a [`Queue`] of type `T` to the `World`.
    fn add_queue<T: Send + 'static>(&mut self);
}

impl AddQueue for World {
    fn add_queue<T: Send + 'static>(&mut self) {
        self.init_resource::<Queue<T>>();
    }
}

impl AddQueue for bevy::prelude::App {
    fn add_queue<T: Send + 'static>(&mut self) {
        self.init_resource::<Queue<T>>();
    }
}
