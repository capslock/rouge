#[cfg(feature = "bevy")]
use bracket_bevy::prelude::{ColorPair, Point, Rect, BLACK, MAGENTA, RGB, WHITE};
#[cfg(not(feature = "bevy"))]
use bracket_lib::terminal::{ColorPair, Point, Rect, VirtualKeyCode, BLACK, MAGENTA, RGB, WHITE};

use crate::{AlignX, AlignY, Context, Interaction, Layout, Ui};

#[cfg(feature = "bevy")]
use bevy_input::prelude::KeyCode;
#[cfg(feature = "bevy")]
type VirtualKeyCode = KeyCode;

pub struct Window<'a> {
    title: String,
    open: Option<&'a mut bool>,
    pos: Option<Point>,
    layout: Layout,
    width: Option<i32>,
}

impl<'a> Window<'a> {
    pub fn new<S: ToString>(title: S) -> Self {
        Self {
            title: title.to_string(),
            open: None,
            pos: None,
            layout: Layout::new(AlignX::Left, AlignY::Top),
            width: None,
        }
    }

    pub fn open(self, open: &'a mut bool) -> Self {
        Self {
            open: Some(open),
            ..self
        }
    }

    pub fn pos(self, point: Point) -> Self {
        Self {
            pos: Some(point),
            ..self
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            width: Some(width),
            ..self
        }
    }

    pub fn layout(self, x: AlignX, y: AlignY) -> Self {
        Self {
            layout: Layout::new(x, y),
            ..self
        }
    }

    pub fn show<R>(self, ctx: &mut Context, add: impl FnOnce(&mut Ui) -> R) -> Option<R> {
        let mut draw_batch = ctx.new_draw_batch();

        let layer = ctx.layer;

        // Add title.
        let title_length = self.title.len() + 9;

        let pos = if let Some(pos) = self.pos {
            pos
        } else {
            Point::new(ctx.screen_rect.x1, ctx.screen_rect.y1)
        };

        let min_width = if let Some(width) = self.width {
            width
        } else {
            0
        };

        let max_width = if let Some(width) = self.width {
            width
        } else {
            ctx.screen_rect.width()
        };

        let rect = Rect {
            x1: pos.x,
            x2: ctx.screen_rect.x2,
            y1: pos.y,
            y2: ctx.screen_rect.y2,
        };

        let layout = Layout::new(AlignX::Left, AlignY::Top);
        let min_rect = layout.allocate_aligned(rect, min_width.max(title_length as i32), 2);

        let mut ui = Ui::new(
            ctx,
            layer + 2000,
            min_rect,
            Rect {
                x1: rect.x1 + 1,
                x2: rect.x1 + max_width.min(ctx.screen_rect.width()) - 1,
                y1: rect.y1 + 1,
                y2: rect.y2 - 1,
            },
            self.layout,
        );

        let r = add(&mut ui);

        // Draw frame.
        draw_batch.draw_box(
            Rect::with_exact(
                ui.min_rect.x1 - 1,
                ui.min_rect.y1 - 1,
                ui.min_rect.x2 + 1,
                ui.min_rect.y2 + 1,
            ),
            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
        );

        draw_batch.print_color(
            Point::new(ui.min_rect.x1, ui.min_rect.y1 - 1),
            &self.title,
            ColorPair::new(RGB::named(MAGENTA), RGB::named(BLACK)),
        );

        draw_batch.print_color(
            Point::new(ui.min_rect.x2 - 7, ui.min_rect.y1 - 1),
            "ESC [x]",
            ColorPair::new(RGB::named(MAGENTA), RGB::named(BLACK)),
        );

        ctx.submit_draw_batch(ui.layer, draw_batch);

        let interaction = Interaction {
            keys: vec![VirtualKeyCode::Space, VirtualKeyCode::Escape],
            click: false,
        };

        for key in ui.interact(Rect::default(), interaction).keys {
            match key {
                VirtualKeyCode::Space | VirtualKeyCode::Escape => {
                    if let Some(open) = self.open {
                        *open = false;
                        return Some(r);
                    }
                }
                _ => {}
            }
        }

        Some(r)
    }
}
