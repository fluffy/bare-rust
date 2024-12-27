#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate core;


use hal;
use hal::led;
use hal::led::Color;



#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    led::set(Color::Red);
    loop {}
}

#[cfg(feature = "std")]
#[test]
fn test_init() {
    hal::init();

    led::set(Color::Green);
}
