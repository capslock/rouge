mod label;
mod selection_list;

pub use label::*;
pub use selection_list::*;

use super::{Ui, UiResult};

/// Trait that widgets should implement.
pub trait Widget {
    /// Called by the [`Ui`] when a `Widget` should build itself. A widget should do
    /// everything it needs to draw itself to the screen and process any
    /// interactions.
    fn ui(self, ui: &mut Ui) -> UiResult;
}
