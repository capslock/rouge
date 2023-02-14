mod label;
mod selection_list;

pub use label::*;
pub use selection_list::*;

use super::{Ui, UiResult};

pub trait Widget {
    fn ui(self, ui: &mut Ui) -> UiResult;
}
