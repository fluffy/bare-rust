#![no_std]

mod board;
pub mod button;
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
    cpu::init();
    
    // always set up clocks first
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // do soon after clock is up so we  can use console
    uart::init1(115_200);
    // do after uart is up
    console::init();

    // Do after GPIO is up
    led::init();
    debug::init();
    button::init();

    // Do last as this starts schedule
    timer::init1();
}

#[inline(never)]
pub fn validate() {
    clock::validate();
    button::validate();
}
