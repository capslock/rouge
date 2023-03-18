mod context;
mod interaction;
mod layout;
mod ui;
mod widget;
mod window;

pub use context::*;
pub use interaction::*;
pub use layout::*;
pub use ui::*;
pub use widget::*;
pub use window::*;

/// The result of adding a [`Widget`] to a [`Ui`].
#[derive(Debug, Default)]
pub struct UiResult {
    pub changed: bool,
}
