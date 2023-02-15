#[cfg(feature = "bevy")]
use bevy_input::prelude::KeyCode;
#[cfg(not(feature = "bevy"))]
use bracket_lib::terminal::VirtualKeyCode;

#[cfg(feature = "bevy")]
type VirtualKeyCode = KeyCode;

#[derive(Debug, Default)]
pub struct Interaction {
    pub keys: Vec<VirtualKeyCode>,
    pub click: bool,
}
