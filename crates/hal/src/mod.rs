#![no_std]

pub mod clock;
pub mod cpu;
pub mod gpio;
pub mod semihost;
pub mod timer;
pub mod uart;
pub mod svd;
pub mod board;

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
    
    // TODO console::init();

    // Do after GPIO is up
    // TODO led::init();
    // TODO debug::init();
    // TODO button::init();

    // Do last as this starts schedule
    timer::init1();
}

#[inline(never)]
pub fn validate() {
    clock::validate();
    // TODO button::validate();
}
