#[cfg(not(feature = "bevy"))]
use std::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy::input::prelude::KeyCode;
#[cfg(feature = "bevy")]
use bevy::input::{prelude::MouseButton, Input};
#[cfg(feature = "bevy")]
use bracket_bevy::DrawBatch;
#[cfg(feature = "bevy")]
use bracket_bevy::{
    prelude::{Point, Rect},
    BracketContext,
};
#[cfg(not(feature = "bevy"))]
use bracket_lib::terminal::{BTerm, DrawBatch, Point, Rect, VirtualKeyCode};
#[cfg(not(feature = "bevy"))]
use object_pool::Reusable;

/// Contains the context for the GUI.
#[cfg(not(feature = "bevy"))]
#[derive(Debug)]
pub struct Context<'a> {
    pub screen_rect: Rect,
    pub layer: usize,
    pub key: Option<VirtualKeyCode>,
    pub mouse: Option<Point>,
    pub clicked: bool,
    marker: PhantomData<&'a ()>,
}

#[cfg(not(feature = "bevy"))]
impl<'a> Context<'a> {
    /// Create a new context.
    pub fn new(ctx: &BTerm, screen_rect: Rect, layer: usize) -> Self {
        Self {
            screen_rect,
            layer,
            key: ctx.key,
            mouse: Some(ctx.mouse_point()),
            clicked: ctx.left_click,
            marker: Default::default(),
        }
    }

    /// Test if any of the given keys were pressed.
    pub fn any_pressed(
        &self,
        inputs: impl IntoIterator<Item = VirtualKeyCode>,
    ) -> Option<VirtualKeyCode> {
        if let Some(key) = self.key {
            if inputs.into_iter().any(|k| k == key) {
                self.key
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Create a new draw batch with the given context.
    pub fn new_draw_batch(&self) -> Reusable<DrawBatch> {
        DrawBatch::new()
    }

    /// Submit ithe provided draw batch.
    pub fn submit_draw_batch(&self, z_order: usize, mut batch: Reusable<DrawBatch>) {
        batch.submit(z_order).expect("Failed to submit batch");
    }
}

/// Contains the context for the GUI.
#[cfg(feature = "bevy")]
pub struct Context<'a> {
    pub ctx: &'a BracketContext,
    pub screen_rect: Rect,
    pub layer: usize,
    pub keys: &'a Input<KeyCode>,
    pub mouse: Option<Point>,
    pub clicked: bool,
    pub mouse_button: &'a Input<MouseButton>,
    pub console: usize,
}

#[cfg(feature = "bevy")]
impl<'a> Context<'a> {
    /// Create a new context.
    pub fn new(
        ctx: &'a BracketContext,
        keys: &'a Input<KeyCode>,
        mouse_button: &'a Input<MouseButton>,
        screen_rect: Rect,
        layer: usize,
        console: usize,
    ) -> Self {
        Self {
            ctx,
            screen_rect,
            layer,
            keys,
            clicked: mouse_button.pressed(MouseButton::Left),
            mouse: Some(ctx.get_mouse_position_for_current_layer()),
            mouse_button,
            console,
        }
    }

    /// Test if any of the given keys were pressed.
    pub fn any_pressed(&self, inputs: impl IntoIterator<Item = KeyCode>) -> Option<KeyCode> {
        inputs.into_iter().find(|k| self.keys.pressed(*k))
    }

    /// Create a new draw batch with the given context.
    pub fn new_draw_batch(&self) -> DrawBatch {
        let mut draw_batch = self.ctx.new_draw_batch();
        draw_batch.target(self.console);
        draw_batch
    }

    /// Submit ithe provided draw batch.
    pub fn submit_draw_batch(&self, z_order: usize, batch: DrawBatch) {
        self.ctx.submit_batch(z_order, batch);
    }
}
