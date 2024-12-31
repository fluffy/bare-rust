#![no_std]

pub mod board;
pub mod clock;
pub mod cpu;
pub mod gpio;
pub mod semihost;
pub mod svd;
pub mod timer;
pub mod uart;

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

    // Do last as this starts timer events
    timer::init1();
}

#[inline(never)]
pub fn validate() {
    clock::validate();
}
