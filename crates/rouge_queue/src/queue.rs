use std::marker::PhantomData;

use bevy::ecs::{prelude::*, system::SystemParam};
use crossbeam::queue::SegQueue;

/// A multi-producer multi-consumer queue.
#[derive(Resource)]
pub struct Queue<T> {
    q: SegQueue<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self {
            q: Default::default(),
        }
    }
}

impl<T> Queue<T> {
    // Create a new queue.
    pub fn new() -> Self {
        Self::default()
    }

    // Test if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }

    // Push an item onto the queue.
    pub fn push(&self, value: T) {
        self.q.push(value)
    }

    // Pop an item off of the queue. Returns `None` if the queue is empty.
    pub fn pop(&self) -> Option<T> {
        self.q.pop()
    }

    // Iterate over items in the queue. This drains the queue, but does not consume
    // the `Queue` itself.
    pub fn iter(&self) -> QueueIter<T> {
        QueueIter::new(self)
    }
}

/// An iterator for `Queue`.
///
/// Created by calling [`Queue::iter`]. See its documentation for more.
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

/// Push items onto a `Queue`.
///
/// Used as a convenience `SystemParam` rather than operating on the underlying
/// [`Queue`] directly.
#[derive(SystemParam)]
pub struct QueueWriter<'w, 's, E>
where
    E: Send + 'static,
{
    queue: Res<'w, Queue<E>>,
    #[system_param(ignore)]
    marker: PhantomData<&'s ()>,
}

impl<'w, 's, E> QueueWriter<'w, 's, E>
where
    E: Send + 'static,
{
    /// Push an item onto the `Queue`.
    pub fn push(&self, value: E) {
        self.queue.push(value)
    }
}

/// Pop item from the `Queue`.
///
/// Used as a convenience `SystemParam` rather than operating on the underlying
/// [`Queue`] directly.
#[derive(SystemParam)]
pub struct QueueReader<'w, 's, E>
where
    E: Send + 'static,
{
    queue: Res<'w, Queue<E>>,
    #[system_param(ignore)]
    marker: PhantomData<&'s ()>,
}

impl<'w, 's, E> QueueReader<'w, 's, E>
where
    E: Send + 'static,
{
    /// Check if the queue is empty without modifying the contents of the `Queue`.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Pop the top item off of the queue.
    pub fn pop(&self) -> Option<E> {
        self.queue.pop()
    }

    /// Iterate over through items in the `Queue`. This drains the `queue`, but
    /// does not consume the underlying `Queue`.
    pub fn iter(&self) -> QueueIter<E> {
        self.queue.iter()
    }
}

/// Trait for pushing items to a [`Queue`].
pub trait PushQueue<T>
where
    T: Send + 'static,
{
    /// Push an item to the `Queue`.
    fn push_queue(&self, value: T);
}

impl<T> PushQueue<T> for World
where
    T: Send + 'static,
{
    /// Push an item to the [`Queue`], panicking if the specified `Queue`
    /// resource does not exist.
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
