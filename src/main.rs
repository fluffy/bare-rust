#![no_std]
#![cfg_attr( not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

use hal;
use hal::debug;
use hal::led;
use hal::led::Color;

mod stack;
mod startup;
//mod hal;

#[cfg(not(feature = "std"))]
#[no_mangle]
#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    my_main();
}

#[cfg(feature = "std")]
fn main() -> () {
    my_main();
}

fn my_main() -> ! {
    hal::init();

    loop {
        led::set(Color::Blue);
        debug::set(0, true);

        // getting 1.630 s on dev
        // getting 0.798 s on rel
        fib(34);

        let _stack_usage = stack::usage();

        debug::set(0, false);
        led::set(Color::Green);

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
