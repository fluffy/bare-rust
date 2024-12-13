#![no_std]
#![feature(concat_idents)]

pub mod hal {
    mod cpu;
    //pub mod clock;
    pub mod gpio;
    pub mod led;
    pub mod clock;
}

#[inline(never)]
pub fn init() {
    hal::clock::init();

    // Do after clock and memory is set up
    hal::gpio::init();

    // Do after GPIO is up
    hal::led::init();
}
