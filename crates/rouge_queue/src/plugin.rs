use bevy_app::Plugin;

use crate::Queue;

type QueueFn = dyn Fn(&mut bevy_app::App) + Send + Sync;

#[derive(Default)]
pub struct QueuePlugin {
    types: Vec<Box<QueueFn>>,
}

impl QueuePlugin {
    pub fn with_queue<T: Send + 'static>(mut self) -> Self {
        self.types.push(Box::new(|app: &mut bevy_app::App| {
            app.init_resource::<Queue<T>>();
        }));
        self
    }
}

impl Plugin for QueuePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        for t in self.types.iter() {
            t(app);
        }
    }
}
