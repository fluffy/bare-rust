#![no_std]
#![no_main]

use core::panic::PanicInfo;

use hal;
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

    led::set(Color::Blue);
    fib(34);
    //fib(34);
    led::set(Color::Green);

    loop {}
}

pub fn fib(x: usize) -> u32 {
    if x > 2 {
        fib(x - 1) + fib(x - 2)
    } else {
        1
    }
}
