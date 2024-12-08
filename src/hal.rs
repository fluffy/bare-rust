#![no_std]
//#![feature(concat_idents)]


pub mod clock;
pub mod gpio;
pub mod led;

mod gen_cpu;


#[inline(never)] 
pub fn init() {
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // Do after GPIO is up
    led::init();
}
