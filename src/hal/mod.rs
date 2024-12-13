#![no_std]
#![feature(concat_idents)]

//pub mod hal {
mod cpu;
//pub mod clock;
pub mod clock;
pub mod debug;
pub mod gpio;
pub mod led;
//}

#[inline(never)]
pub fn init() {
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // Do after GPIO is up
    led::init();
    debug::init();
}
