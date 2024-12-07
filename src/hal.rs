#![no_std]

pub mod clock;
pub mod gpio;
pub mod led;

pub fn init() {
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // Do after GPIO is up
    led::init();
}
