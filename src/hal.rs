#![no_std]

 //mod crate::gpio;
 //mod crate::led;

//use crate::led;
//use crate::gpio;

pub mod led;
pub mod gpio;

    pub fn init() {
gpio::init();
        led::init();
    }

