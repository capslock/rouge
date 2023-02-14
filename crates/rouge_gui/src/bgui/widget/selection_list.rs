use bracket_lib::prelude::*;

use crate::{Interaction, Label, Ui, UiResult, Widget};

pub struct Paginate<'a, T: Copy> {
    items: Vec<(T, String)>,
    paginate: usize,
    page: &'a mut usize,
    total_items: usize,
}

impl<'a, T: Copy> Paginate<'a, T> {
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<S: ToString>(mut self, key: T, text: S) -> Self {
        self.items.push((key, text.to_string()));
        self
    }

    pub fn add_list<S: ToString>(mut self, iter: impl Iterator<Item = (T, impl ToString)>) -> Self {
        self.items.extend(iter.map(|(k, s)| (k, s.to_string())));
        self
    }

    pub fn selected(self, selection: &'a mut Option<T>) -> Self {
        Self {
            selected: Some(selection),
            ..self
        }
    }

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
            0 => Some(VirtualKeyCode::A),
            1 => Some(VirtualKeyCode::B),
            2 => Some(VirtualKeyCode::C),
            3 => Some(VirtualKeyCode::D),
            4 => Some(VirtualKeyCode::E),
            5 => Some(VirtualKeyCode::F),
            6 => Some(VirtualKeyCode::G),
            7 => Some(VirtualKeyCode::H),
            8 => Some(VirtualKeyCode::I),
            9 => Some(VirtualKeyCode::J),
            10 => Some(VirtualKeyCode::K),
            11 => Some(VirtualKeyCode::L),
            12 => Some(VirtualKeyCode::M),
            13 => Some(VirtualKeyCode::N),
            14 => Some(VirtualKeyCode::O),
            15 => Some(VirtualKeyCode::P),
            16 => Some(VirtualKeyCode::Q),
            17 => Some(VirtualKeyCode::R),
            18 => Some(VirtualKeyCode::S),
            19 => Some(VirtualKeyCode::T),
            20 => Some(VirtualKeyCode::U),
            21 => Some(VirtualKeyCode::V),
            22 => Some(VirtualKeyCode::W),
            23 => Some(VirtualKeyCode::X),
            24 => Some(VirtualKeyCode::Y),
            25 => Some(VirtualKeyCode::Z),
            _ => None,
        }
    }
}

impl<'a, T: Copy> Widget for SelectionList<'a, T> {
    fn ui(mut self, ui: &mut Ui) -> UiResult {
        let mut draw_batch = DrawBatch::new();

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

        draw_batch
            .submit(ui.layer)
            .expect("Failed to submit batch!");

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
