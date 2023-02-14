use bracket_lib::prelude::*;

#[derive(Debug)]
pub struct Context {
    pub screen_rect: Rect,
    pub layer: usize,
    pub key: Option<VirtualKeyCode>,
    pub mouse: Option<Point>,
    pub clicked: bool,
}

impl Context {
    pub fn new(ctx: &BTerm, screen_rect: Rect, layer: usize) -> Self {
        Self {
            screen_rect,
            layer,
            key: ctx.key,
            mouse: Some(ctx.mouse_point()),
            clicked: ctx.left_click,
        }
    }
}
