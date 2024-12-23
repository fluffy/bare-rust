#![no_std]

mod cpu;
//pub mod clock;
mod board;
pub mod clock;
pub mod debug;
pub mod gpio;
pub mod led;

#[inline(never)]
pub fn init() {
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // Do after GPIO is up
    led::init();
    debug::init();
}
