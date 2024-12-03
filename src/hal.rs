#![no_std]

pub mod gpio;
pub mod led;

pub fn init() {
    gpio::init();
    led::init();
}
