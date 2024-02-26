#[cfg(feature = "bevy")]
use bevy::input::prelude::KeyCode;
#[cfg(feature = "bevy")]
use bracket_bevy::prelude::{Rect, TextBuilder, WHITE, YELLOW};
#[cfg(not(feature = "bevy"))]
use bracket_lib::terminal::{letter_to_option, Rect, TextBuilder, VirtualKeyCode, WHITE, YELLOW};

#[cfg(feature = "bevy")]
type VirtualKeyCode = KeyCode;

use crate::{Interaction, Label, Ui, UiResult, Widget};

/// A widget that paginates a list of items.
pub struct Paginate<'a, T: Copy> {
    items: Vec<(T, String)>,
    paginate: usize,
    page: &'a mut usize,
    total_items: usize,
}

impl<'a, T: Copy> Paginate<'a, T> {
    /// Create a new `Paginate` widget with the given list of items and `paginate`
    /// count.
    pub fn new(
        items: impl Iterator<Item = (T, impl ToString)>,
        paginate: usize,
        page: &'a mut usize,
    ) -> Self {
        let items: Vec<_> = items.collect();
        let total_items = items.len();
        let mut start = *page * paginate;
        if start > items.len() {
            start = 0;
            *page = 0;
        }
        Self {
            items: items
                .into_iter()
                .skip(start)
                .take(paginate)
                .map(|(k, s)| (k, s.to_string()))
                .collect(),
            paginate,
            page,
            total_items,
        }
    }

    /// Get an iterator to the items on the current page.
    pub fn items(&self) -> impl Iterator<Item = &(T, String)> {
        self.items.iter()
    }
}

impl<'a, T: Copy> Widget for Paginate<'a, T> {
    fn ui(self, ui: &mut Ui) -> UiResult {
        if self.total_items < self.paginate {
            return UiResult::default();
        }
        let max_pages =
            (self.total_items / self.paginate) + (self.total_items % self.paginate).min(1);
        let mut text = TextBuilder::empty();
        text.fg(YELLOW)
            .append(&format!("{}/{} TAB for more", *self.page + 1, max_pages));
        ui.add(Label::new(text));
        let interaction = Interaction {
            click: false,
            keys: vec![VirtualKeyCode::Tab],
        };
        let interacted = ui.interact(Rect::default(), interaction);
        for key in interacted.keys {
            if key == VirtualKeyCode::Tab {
                *self.page = (*self.page + 1) % max_pages;
                return UiResult::default();
            }
        }
        UiResult::default()
    }
}

/// A widget which displays a list of options that can be selected.
pub struct SelectionList<'a, T: Copy> {
    items: Vec<(T, String)>,
    selected: Option<&'a mut Option<T>>,
    paginate: Option<Paginate<'a, T>>,
}

impl<'a, T: Copy> Default for SelectionList<'a, T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
            selected: Default::default(),
            paginate: Default::default(),
        }
    }
}

impl<'a, T: Copy> SelectionList<'a, T> {
    /// Create a new `SelectionList` widget.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a single option to the list.
    pub fn add<S: ToString>(mut self, key: T, text: S) -> Self {
        self.items.push((key, text.to_string()));
        self
    }

    /// Add multiple options to the list.
    pub fn add_list<S: ToString>(mut self, iter: impl Iterator<Item = (T, impl ToString)>) -> Self {
        self.items.extend(iter.map(|(k, s)| (k, s.to_string())));
        self
    }

    /// Provide a `selection` parameter that will hold the user's selection, if any.
    pub fn selected(self, selection: &'a mut Option<T>) -> Self {
        Self {
            selected: Some(selection),
            ..self
        }
    }

    /// Paginate the list.
    pub fn paginate(
        self,
        items: impl Iterator<Item = (T, impl ToString)>,
        paginate: usize,
        page: &'a mut usize,
    ) -> Self {
        Self {
            paginate: Some(Paginate::new(items, paginate, page)),
            ..self
        }
    }

    /// For A-Z menus, translates the keys A through Z into 0..25
    fn option_to_keycode(option: i32) -> Option<VirtualKeyCode> {
        match option {
            0 => Some(VirtualKeyCode::KeyA),
            1 => Some(VirtualKeyCode::KeyB),
            2 => Some(VirtualKeyCode::KeyC),
            3 => Some(VirtualKeyCode::KeyD),
            4 => Some(VirtualKeyCode::KeyE),
            5 => Some(VirtualKeyCode::KeyF),
            6 => Some(VirtualKeyCode::KeyG),
            7 => Some(VirtualKeyCode::KeyH),
            8 => Some(VirtualKeyCode::KeyI),
            9 => Some(VirtualKeyCode::KeyJ),
            10 => Some(VirtualKeyCode::KeyK),
            11 => Some(VirtualKeyCode::KeyL),
            12 => Some(VirtualKeyCode::KeyM),
            13 => Some(VirtualKeyCode::KeyN),
            14 => Some(VirtualKeyCode::KeyO),
            15 => Some(VirtualKeyCode::KeyP),
            16 => Some(VirtualKeyCode::KeyQ),
            17 => Some(VirtualKeyCode::KeyR),
            18 => Some(VirtualKeyCode::KeyS),
            19 => Some(VirtualKeyCode::KeyT),
            20 => Some(VirtualKeyCode::KeyU),
            21 => Some(VirtualKeyCode::KeyV),
            22 => Some(VirtualKeyCode::KeyW),
            23 => Some(VirtualKeyCode::KeyX),
            24 => Some(VirtualKeyCode::KeyY),
            25 => Some(VirtualKeyCode::KeyZ),
            _ => None,
        }
    }
}

#[cfg(feature = "bevy")]
fn letter_to_option(option: VirtualKeyCode) -> i32 {
    match option {
        VirtualKeyCode::KeyA => 0,
        VirtualKeyCode::KeyB => 1,
        VirtualKeyCode::KeyC => 2,
        VirtualKeyCode::KeyD => 3,
        VirtualKeyCode::KeyE => 4,
        VirtualKeyCode::KeyF => 5,
        VirtualKeyCode::KeyG => 6,
        VirtualKeyCode::KeyH => 7,
        VirtualKeyCode::KeyI => 8,
        VirtualKeyCode::KeyJ => 9,
        VirtualKeyCode::KeyK => 10,
        VirtualKeyCode::KeyL => 11,
        VirtualKeyCode::KeyM => 12,
        VirtualKeyCode::KeyN => 13,
        VirtualKeyCode::KeyO => 14,
        VirtualKeyCode::KeyP => 15,
        VirtualKeyCode::KeyQ => 16,
        VirtualKeyCode::KeyR => 17,
        VirtualKeyCode::KeyS => 18,
        VirtualKeyCode::KeyT => 19,
        VirtualKeyCode::KeyU => 20,
        VirtualKeyCode::KeyV => 21,
        VirtualKeyCode::KeyW => 22,
        VirtualKeyCode::KeyX => 23,
        VirtualKeyCode::KeyY => 24,
        VirtualKeyCode::KeyZ => 25,
        _ => -1,
    }
}

impl<'a, T: Copy> Widget for SelectionList<'a, T> {
    fn ui(mut self, ui: &mut Ui) -> UiResult {
        let draw_batch = ui.ctx.new_draw_batch();

        let mut buf = [0; 1];

        let items = if let Some(ref paginate) = self.paginate {
            if paginate.total_items > paginate.paginate {
                // FIXME: This won't work for other layout directions.
                ui.min_rect.y2 += paginate.paginate as i32;
            }
            paginate.items().cloned().collect()
        } else {
            self.items
        };

        if let Some(paginate) = self.paginate.take() {
            ui.add(paginate);
        }

        for (i, (_k, s)) in items.iter().enumerate() {
            let mut text = TextBuilder::empty();
            text.fg(WHITE)
                .append("(")
                .fg(YELLOW)
                .append(((i as u8 + 97) as char).encode_utf8(&mut buf))
                .fg(WHITE)
                .append(") ")
                .append(s);
            ui.add(Label::new(text));
        }

        ui.ctx.submit_draw_batch(ui.layer, draw_batch);

        let interaction = Interaction {
            click: false,
            keys: (0..items.len())
                .map(|i| Self::option_to_keycode(i as i32).unwrap())
                .collect(),
        };

        let interacted = ui.interact(Rect::default(), interaction);

        for key in interacted.keys {
            let option = letter_to_option(key);
            let key = &items[option as usize].0;
            if let Some(selected) = self.selected {
                *selected = Some(*key);
                return UiResult::default();
            }
        }

        UiResult::default()
    }
}
