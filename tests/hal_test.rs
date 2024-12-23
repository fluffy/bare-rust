#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate core;

#[cfg(target_arch = "arm")]
use core::panic::PanicInfo;

use hal;
use hal::led;
use hal::led::Color;

#[cfg(target_arch = "arm")]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

//#[cfg(test)]
#[test]
fn test_init() {
    hal::init();

    led::set(Color::Green);
}
