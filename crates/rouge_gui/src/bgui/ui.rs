use bracket_lib::terminal::Rect;

use crate::{Context, Interaction, Label, Layout, SelectionList, UiResult, Widget};

/// A struct that manages the current UI.
pub struct Ui<'a> {
    pub ctx: &'a Context<'a>,
    pub max_rect: Rect,
    pub min_rect: Rect,
    pub cursor: Rect,
    pub layout: Layout,
    pub layer: usize,
}

impl<'a> Ui<'a> {
    /// Create a new `Ui`. Usually doesn't need to be called manually as a `Ui` will
    /// be provided for you.
    pub fn new(
        ctx: &'a Context<'a>,
        layer: usize,
        min_rect: Rect,
        max_rect: Rect,
        layout: Layout,
    ) -> Self {
        Self {
            ctx,
            min_rect,
            max_rect,
            cursor: Rect {
                x1: max_rect.x1,
                x2: max_rect.x2,
                y1: max_rect.y1,
                y2: max_rect.y2,
            },
            layout,
            layer,
        }
    }

    /// Allocate some space in the current Ui.
    pub fn allocate(&mut self, x: i32, y: i32) -> Rect {
        let rect = self.layout.allocate_aligned(self.cursor, x, y);
        self.cursor.y1 = rect.y2;
        self.min_rect.x1 = self.min_rect.x1.min(rect.x1);
        self.min_rect.x2 = self.min_rect.x2.max(rect.x2);
        self.min_rect.y1 = self.min_rect.y1.min(rect.y1);
        self.min_rect.y2 = self.min_rect.y2.max(rect.y2);
        rect
    }

    /// Specify an interaction in the current Ui. Specifying `rect` allows for
    /// mouse clicks to be captured in a specific area. `interaction` specifies
    /// which kind of interactions should be tested.
    pub fn interact(&mut self, rect: Rect, interaction: Interaction) -> Interaction {
        let mut interacted = Interaction::default();
        if interaction.click
            && self.ctx.clicked
            && self.ctx.mouse.is_some()
            && rect.point_in_rect(self.ctx.mouse.unwrap().into())
        {
            interacted.click = true;
            //self.ctx.clicked = false;
            //self.ctx.mouse = None;
        }

        if let Some(key) = self.ctx.any_pressed(interaction.keys.iter().copied()) {
            interacted.keys.push(key);
        }

        interacted
    }

    /// Add a [`Widget`] to the `Ui`.
    pub fn add(&mut self, widget: impl Widget) -> UiResult {
        widget.ui(self)
    }

    /// Add a [`Label`] widget to the `Ui`.
    pub fn label<T: ToString>(&mut self, text: T) -> UiResult {
        self.add(Label::from_string(text))
    }

    /// Add a [`SelectionList`] widget to the `Ui`.
    pub fn selection_list<T: Copy>(&mut self) -> UiResult {
        self.add(SelectionList::<T>::new())
    }
}
