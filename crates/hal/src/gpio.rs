//! # GPIO Module
//!
//! This module provides functionality for initializing and controlling
//! General-Purpose Input/Output (GPIO) pins.
//! It includes methods for setting pins as input or output, configuring
//! pull-up or pull-down resistors, and reading or writing pin states.
//!
//! ## Structures
//!
//! - `Pin`: Represents a GPIO pin and provides methods to configure and control it.
//!
//! ## Functions
//!
//! - `init`: Initializes the GPIO peripheral by enabling the necessary clocks.
//!
//! ## Methods for `Pin`
//!
//! - `new`: Creates a new `Pin` instance.
//! - `output`: Configures the pin as an output.
//! - `input`: Configures the pin as an input with a pull-down resistor.
//! - `pulldown`: Configures the pin with a pull-down resistor.
//! - `pullup`: Configures the pin with a pull-up resistor.
//! - `low`: Sets the pin state to low.
//! - `high`: Sets the pin state to high.
//! - `read`: Reads the current state of the pin.
//!
//! ## Usage
//!
//! This module is intended for low-level hardware interaction and should be used with caution.
//! It provides direct access to hardware registers,
//! which can lead to undefined behavior if used incorrectly.
//!
//! ## Example
//!
//! ```rust
//! use crate::hal::gpio::{self, Pin};
//! use crate::hal::cpu;
//! use crate::hal::clock;
//!
//! fn main() {
//!     // Initialize things
//!     cpu::init();
//!     clock::init();
//!     gpio::init();
//!
//!     // Create a new pin instance
//!     let pin = Pin::new(cpu::GPIO_A, 5);
//!
//!     // Configure the pin as output and set it high
//!     pin.output();
//!     pin.high();
//!
//!     // Read the pin state
//!     let state = pin.read();
//!     println!("Pin state: {}", state);
//! }
//! ```

use core::ptr;

use super::cpu;
pub use super::cpu::*;
//pub use super::svd::*;

pub fn init() {
    cpu::write!(RCC.ahb1enr[GPIOAEN;1], 1 );
    cpu::write!(RCC.ahb1enr[GPIOBEN;1], 1 );
    cpu::write!(RCC.ahb1enr[GPIOCEN;1], 1 );
}

#[derive(Copy, Clone)]
pub struct Pin(pub *mut cpu::GpioReg, pub u8);

impl Pin {
    #[allow(dead_code)]
    pub fn new(gpio: *mut cpu::GpioReg, p: u8) -> Pin {
        assert!(p < 16);
        return Pin(gpio, p);
    }

    pub fn output(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set mode to output
        cpu::write!( gpio.moder[pin_num*2;2], 0b01);

        // set output as low
        cpu::write!( gpio.odr[pin_num*1;1], 0b0);

        // set as push-pull
        cpu::write!( gpio.otyper[pin_num*1;1], 0b0);

        // set no pull up , no pull down
        cpu::write!( gpio.pupdr[pin_num*2;2], 0b00);

        // set speed to slow
        cpu::write!( gpio.ospeedr[pin_num*2;2], 0b00);
    }

    pub fn input(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set to pull down
        cpu::write!( gpio.pupdr[pin_num*2;2], 0b10);

        // set mode to input
        cpu::write!( gpio.moder[pin_num*2;2], 0b00);
    }

    #[allow(dead_code)]
    pub fn pulldown(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set to pull down
        cpu::write!(gpio.pupdr[pin_num * 2; 2], 0b10);
    }

    #[allow(dead_code)]
    pub fn pullup(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set to pull up
        cpu::write!(gpio.pupdr[pin_num * 2; 2], 0b01);
    }

    pub fn low(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        cpu::write!(gpio.bsrr, 0b1 << (pin_num + 16));
    }

    pub fn high(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        cpu::write!(gpio.bsrr, 0b1 << (pin_num + 0));
    }

    pub fn read(&self) -> bool {
        let gpio = self.0;
        let pin_num: u32 = self.1 as u32;

        let val = cpu::read!( gpio.idr[pin_num*1;1] );
        val != 0
    }
}
