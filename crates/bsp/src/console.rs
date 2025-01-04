//! # Console Module
//!
//! This module provides functionality for printing debug information to the console.
//!
//! ## Traits
//!
//! - `Print`: A trait for printing data to the console.
//! Implemented for various data types such as `[u8]`, and `u64`
//!
//! ## Example
//!
//! ```rust
//!  use bsp::BSP;
//!  use bsp::console::Print;
//!  let mut bsp = BSP::new();
//!  bsp.init();
//!
//!  let message: &[u8] = b"Hello, world!";
//!  message.print_console();
//!
//!  let number: u64 = 42;
//!  number.print_console();
//! ```

//#[cfg(not(feature = "std"))]
//use core::arch::asm;

extern crate hal;

//use crate::board;
use hal::uart;

#[cfg(feature = "std")]
extern crate std;

pub struct Console {}

impl Console {
    #[inline(never)]
    pub fn new() -> Self {
        Console {}
    }

    #[inline(never)]
    pub fn init(&self) {}
}

pub trait Print {
    fn print_console(&self);
}

impl Print for [u8] {
    fn print_console(&self) {
        let s = self;

        if cfg!(feature = "std") {
            #[cfg(feature = "std")]
            for c in s {
                std::print!("{}", *c as char);
            }
            return;
        }
        for c in s {
            uart::write1(*c);
        }
    }
}

impl Print for u64 {
    fn print_console(&self) {
        let v = *self;
        let mut num = v;
        let mut buffer = [0u8; 20]; // u64 max is 20 digits
        let mut len: usize = 0;

        if num == 0 {
            buffer[len] = b'0';
            len += 1;
        } else {
            while num > 0 {
                buffer[len] = (num % 10) as u8 + b'0';
                num /= 10;
                len += 1;
            }
        }

        let slice = &mut buffer[0..len];
        slice.reverse();

        slice.print_console();
    }
}

impl Print for bool {
    fn print_console(&self) {
        if *self {
            b"true".print_console();
        } else {
            b"false".print_console();
        }
    }
}

impl Print for u32 {
    fn print_console(&self) {
        let v = *self as u64;
        v.print_console();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_console_u8_slice() {
        let message: &[u8] = b"OK";
        message.print_console();
    }

    #[test]
    fn test_print_console_u64() {
        let number: u64 = 42;
        number.print_console();
    }

    #[test]
    fn test_print_console_bool() {
        let value_true: bool = true;
        value_true.print_console();

        let value_false: bool = false;
        value_false.print_console();
    }

    #[test]
    fn test_print_console_u32() {
        let number: u32 = 123;
        number.print_console();
    }
}
