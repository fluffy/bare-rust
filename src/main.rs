#![no_std]
#![no_main]

use core::panic::PanicInfo;

use hal;
use hal::debug;
use hal::led;
use hal::led::Color;

mod startup;
//mod hal;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    hal::init();

    loop {
        led::set(Color::Blue);
        debug::set(0, true);

        // getting 1.630 seconds
        fib(34);

        debug::set(0, false);
        led::set(Color::Red);

        fib(33);
    }
}

pub fn fib(x: usize) -> u32 {
    if x > 2 {
        fib(x - 1) + fib(x - 2)
    } else {
        1
    }
}
