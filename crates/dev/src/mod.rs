#![no_std]

extern crate hal;

pub mod button;
pub mod console;
pub mod debug;
pub mod led;

#[inline(never)]
pub fn init() {
    hal::init();

    // do early so other modules can use it
    console::init();

    led::init();
    debug::init();
    button::init();
}

#[inline(never)]
pub fn validate() {
    hal::validate();

    button::validate();
}
