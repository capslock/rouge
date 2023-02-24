use bevy::app::Plugin;

use crate::Queue;

type QueueFn = dyn Fn(&mut bevy::app::App) + Send + Sync;

#[derive(Default)]
pub struct QueuePlugin {
    types: Vec<Box<QueueFn>>,
}

impl QueuePlugin {
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
