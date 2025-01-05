#![no_std]

//! # Board Support Package (BSP)
//!
//! This module provides the Board Support Package (BSP) for the project,
//! which includes initialization and validation of various hardware
//! components such as buttons, console, debug interface, and LEDs.
//!
//! ## Modules
//!
//! - `buttons`: Manages the hardware input button
//! - `console`: Manages the console interface to print debug messages.
//! - `debug`: Manages the debug interface that outputs signals to the debug pins
//! - `led`: Manages the output of the LED on the board
//!
//! ## Structs
//!
//! - `BSP`: Represents the Board Support Package,
//!    providing methods to initialize and validate the hardware components.
//!
//! ## Usage
//!
//! To use this module, create an instance of `BSP` and
//! call the `init` method to initialize the hardware components.
//!
//! ## Example
//!
//! ```rust
//!  use bsp::BSP;
//!  use bsp::led;
//!
//!  let mut bsp = BSP::new();
//!  bsp.init();
//!
//!  led::set(led::Color::Green);
//! ```
//!

extern crate hal;

pub mod board;
pub mod buttons;
pub mod console;
pub mod debug;
pub mod led;
pub mod battery;
mod keyboard;

pub struct BSP {
    pub buttons: buttons::Buttons,
    pub console: console::Console,
    pub debug: debug::Debug,
    pub led: led::Led,
}

impl BSP {
    pub fn new() -> Self {
        BSP {
            buttons: buttons::Buttons::new(),
            console: console::Console::new(),
            debug: debug::Debug::new(),
            led: led::Led::new(),
        }
    }

    pub fn init(&mut self) {
        let tx_pin = board::info::CONSOLE_TX;
        let rx_pin = board::info::CONSOLE_RX;
        hal::init(board::info::CLOCK_HSE_FREQ, tx_pin, rx_pin);

        // setup console early so other modules can use it
        self.console.init();

        self.led.init();
        self.debug.init();
        self.buttons.init();
    }

    pub fn validate(&self) {
        hal::validate();

        self.buttons.validate();
    }
}
