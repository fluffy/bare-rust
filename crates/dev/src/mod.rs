#![no_std]

//#[macro_use]
extern crate hal;

//use hal::cpu;
//use hal::clock;
//use hal::gpio;
//use hal::uart;


pub mod button;
pub mod console;
pub mod debug;
pub mod led;


#[inline(never)]
pub fn init() {
    
    hal::init();
    
    // todo cpu::init();
    
    // always set up clocks first
    // todo clock::init();

    // Do after clock and memory is set up
    // todo gpio::init();

    // do soon after clock is up so we  can use console
    // todo uart::init1(115_200);
    // do after uart is up
    console::init();

    // Do after GPIO is up
    led::init();
    debug::init();
    button::init();

    // Do last as this starts schedule
    // todo timer::init1();
}

#[inline(never)]
pub fn validate() {
    hal::validate();
    
    // todo clock::validate();
    button::validate();
}
