#![no_std]

mod gpio;
pub mod led;

pub fn init() {
    led::init();
}
