#![no_std]
#![no_main]

use core::panic::PanicInfo;

use hal;
use hal::led;
use hal::led::Color;

mod startup;
mod hal;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[export_name = "main"]
pub extern "C" fn main() -> ! {
    hal::init();

    led::set( Color::Green);

    loop {}
}
