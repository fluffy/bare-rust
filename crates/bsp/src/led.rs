//! # LED Module
//!
//! This module provides functionality for controlling the LED on the board.
//! It includes methods for initializing the LED and setting its colors.
//!
//! ## Enums
//!
//! - `Color`: An enumeration of possible LED colors (Black, White, Red, Green, Blue).
//!
//! ## Functions
//!
//! - `set`: Sets the color of the LEDs based on the provided `Color` enum value.
//!
//! ## Example
//!
//! ```rust
//!  use bsp::BSP;
//!  use bsp::led;
//!  let mut bsp = BSP::new();
//!  bsp.init();
//!
//!  led::set(led::Color::Blue);
//!  led::set(led::Color::Black); // turn off LED
//! ```
//!

extern crate hal;

use crate::board;

#[cfg(feature = "std")]
extern crate std;

pub struct Led {}

impl Led {
    #[inline(never)]
    pub fn new() -> Self {
        Led {}
    }

    #[inline(never)]
    pub fn init(&self) {
        board::info::LED_RED_PIN.output();
        board::info::LED_GREEN_PIN.output();
        board::info::LED_BLUE_PIN.output();

        set(Color::Black);
    }
}

/// Represents the possible colors for the LED.
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}

/// Sets the color of the LED based on the provided `Color` enum value.
pub fn set(c: Color) {
    match c {
        Color::Red => {
            board::info::LED_RED_PIN.high();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();

            #[cfg(feature = "std")]
            std::print!("LED: RED\r\n");
        }
        Color::Green => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.high();
            board::info::LED_BLUE_PIN.low();

            #[cfg(feature = "std")]
            std::print!("LED: GREEN\r\n");
        }
        Color::Blue => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.high();

            #[cfg(feature = "std")]
            std::print!("LED: BLUE\r\n");
        }
        Color::Black => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();

            #[cfg(feature = "std")]
            std::print!("LED: BLACK\r\n");
        }
        Color::White => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();

            #[cfg(feature = "std")]
            std::print!("LED: WHITE\r\n");
        }
    }
}
