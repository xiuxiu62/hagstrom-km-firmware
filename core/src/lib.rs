pub mod action;
mod emulator;
pub mod error;

// pub use action::{KeyCode, MouseAction, ScrollDirection, ScrollMagnitude};
pub use emulator::Emulator;

pub const FLUSH: u8 = 0x38;
