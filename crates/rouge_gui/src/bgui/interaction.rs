use bracket_lib::terminal::VirtualKeyCode;

#[derive(Debug, Default)]
pub struct Interaction {
    pub keys: Vec<VirtualKeyCode>,
    pub click: bool,
}
