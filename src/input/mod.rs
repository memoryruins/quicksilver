extern crate glutin;

mod keyboard;
mod mouse;
mod state;
mod viewport;

pub use glutin::VirtualKeyCode as Key;

pub use self::keyboard::Keyboard;
pub use self::mouse::{Mouse, MouseBuilder};
pub use self::state::ButtonState;
pub use self::viewport::{Viewport, ViewportBuilder};
