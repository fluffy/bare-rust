//! # Keyboard Module
//!
//! This module provides functionality for interacting and retrieving key presses.
//!
//! ## Functions
//!
//! - `new`: Creates a new instance of the `Keyboard` struct.
//! - `init`: Initializes the keyboard interface.
//! - `get_key`: Retrieves the key that has been pressed.
//!
//!

extern crate hal;

#[cfg(feature = "std")]
extern crate std;

pub struct Keyboard {}

impl crate::keyboard::Keyboard {
    #[inline(never)]
    pub fn new() -> Self {
        crate::keyboard::Keyboard {}
    }

    #[inline(never)]
    pub fn init(&self) {}

    /// Gets the key that has been pressed. This will return just one key at a time.
    ///
    /// # Returns
    /// A `u8` representing the key pressed. Returns 0 if no key has been pressed.

    pub fn get_key(&self) -> u8 {
        // returns 0 if no key has been pressed
        0
    }
}
