#[cfg(feature = "bevy")]
use bracket_bevy::prelude::{Point, Rect, TextBlock, TextBuilder, WHITE};
#[cfg(not(feature = "bevy"))]
use bracket_lib::terminal::{Point, Rect, TextBlock, TextBuilder, WHITE};

use crate::{Ui, UiResult, Widget};

pub struct Label<T> {
    text: T,
}

impl Label<TextBuilder> {
    pub fn new(text: TextBuilder) -> Self {
        Self { text }
    }
}

impl Widget for Label<TextBuilder> {
    fn ui(self, ui: &mut Ui) -> UiResult {
        let mut draw_batch = ui.ctx.new_draw_batch();

        let height = 1;

        let mut block =
            TextBlock::new_autoexpand(ui.cursor.x1, ui.cursor.y1, ui.cursor.width(), height);

        block.print(&self.text).expect("Failed to print");
        let min_rect = block.get_min_bounding_rect();
        let (w, h) = (min_rect.width(), min_rect.height());

        let rect = ui.allocate(w, h);
        block.set_origin(Point::new(rect.x1, rect.y1));

        let clip_rect = Rect::with_size(0, 0, w, h);

        block.render_to_draw_batch_clip(&mut draw_batch, &clip_rect);

        ui.ctx.submit_draw_batch(ui.layer, draw_batch);

        UiResult::default()
    }
}

impl Label<String> {
    pub fn from_string<T: ToString>(text: T) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Widget for Label<String> {
    fn ui(self, ui: &mut Ui) -> UiResult {
        let mut draw_batch = ui.ctx.new_draw_batch();

        // Estimate the height.
        let height = self.text.len() / ui.cursor.width() as usize
            + (self.text.len() % ui.cursor.width() as usize).min(1);

        let mut block =
            TextBlock::new_autoexpand(ui.cursor.x1, ui.cursor.y1, ui.cursor.width(), height as i32);

        let mut buf = TextBuilder::empty();

        buf.fg(WHITE);
        buf.line_wrap(&self.text);
        block.print(&buf).expect("Failed to print");

        let min_rect = block.get_min_bounding_rect();
        let (w, h) = (min_rect.width(), min_rect.height());

        let rect = ui.allocate(w, h);
        block.set_origin(Point::new(rect.x1, rect.y1));

        let clip_rect = Rect::with_size(0, 0, w, h);

        block.render_to_draw_batch_clip(&mut draw_batch, &clip_rect);
        ui.ctx.submit_draw_batch(ui.layer, draw_batch);

        UiResult::default()
    }
}
