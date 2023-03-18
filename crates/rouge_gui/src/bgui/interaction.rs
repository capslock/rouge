#[cfg(feature = "bevy")]
use bevy::input::prelude::KeyCode;
#[cfg(not(feature = "bevy"))]
use bracket_lib::terminal::VirtualKeyCode;

#[cfg(feature = "bevy")]
type VirtualKeyCode = KeyCode;

/// Specifies what kind of interaction has occurred or should occur.
#[derive(Debug, Default)]
pub struct Interaction {
    pub keys: Vec<VirtualKeyCode>,
    pub click: bool,
}
