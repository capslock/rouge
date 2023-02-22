use std::marker::PhantomData;

use bevy::ecs::{prelude::*, system::SystemParam};
use crossbeam::queue::SegQueue;

#[derive(Resource)]
pub struct Queue<T> {
    q: SegQueue<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Send> Sync for Queue<T> {}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            q: Default::default(),
        }
    }

    pub fn push(&self, value: T) {
        self.q.push(value)
    }

    pub fn pop(&self) -> Option<T> {
        self.q.pop()
    }

    pub fn iter(&self) -> QueueIter<T> {
        QueueIter::new(self)
    }
}

pub struct QueueIter<'a, T> {
    q: &'a Queue<T>,
}

impl<'a, T> QueueIter<'a, T> {
    fn new(q: &'a Queue<T>) -> Self {
        Self { q }
    }
}

impl<'a, T> Iterator for QueueIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.q.pop()
    }
}

#[derive(SystemParam)]
pub struct QueueWriter<'w, 's, E>
where
    E: Send + 'static,
{
    pub queue: Res<'w, Queue<E>>,
    #[system_param(ignore)]
    marker: PhantomData<&'s ()>,
}

impl<'w, 's, E> QueueWriter<'w, 's, E>
where
    E: Send + 'static,
{
    pub fn push(&self, value: E) {
        self.queue.push(value)
    }
}

#[derive(SystemParam)]
pub struct QueueReader<'w, 's, E>
where
    E: Send + 'static,
{
    pub queue: Res<'w, Queue<E>>,
    #[system_param(ignore)]
    marker: PhantomData<&'s ()>,
}

impl<'w, 's, E> QueueReader<'w, 's, E>
where
    E: Send + 'static,
{
    pub fn pop(&self) -> Option<E> {
        self.queue.pop()
    }

    pub fn iter(&self) -> QueueIter<E> {
        self.queue.iter()
    }
}

pub trait PushQueue<T>
where
    T: Send + 'static,
{
    fn push_queue(&self, value: T);
}

impl<T> PushQueue<T> for World
where
    T: Send + 'static,
{
    fn push_queue(&self, value: T) {
        match self.get_resource::<Queue<T>>() {
            Some(queue) => queue.push(value),
            None => panic!(
                "Unable to push to queue `{}`\n\tQueue must be added to the world with `insert_resource()`.",
                std::any::type_name::<T>()
            ),
        }
    }
}
