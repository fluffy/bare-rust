#![no_std]

mod board;
mod clock;
pub mod console;
mod cpu;
pub mod debug;
mod gpio;
pub mod led;
pub mod semihost;
pub mod timer;
mod uart;

#[inline(never)]
pub fn init() {
    // always set up clocks first
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // Do after GPIO is up
    led::init();

    // Do after LED is up
    debug::init();
    uart::init1(115_200);

    // do after uart is up
    console::init();

    // Do last as this starts schedule
    timer::init1();
}

pub fn validate() {
    clock::validate();
}
