use bracket_lib::prelude::*;

#[derive(Debug, Default)]
pub struct Interaction {
    pub keys: Vec<VirtualKeyCode>,
    pub click: bool,
}
