#![no_std]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

use hal;
use hal::led;
use hal::led::Color;
use hal::{debug, uart};

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

    //#[cfg(feature = "exit")]
    //hal::clock::validate();

    uart::write1(b"Starting\r\n\0");

    loop {
        led::set(Color::Blue);
        debug::set(0, true);

        // getting 1.630 s on dev
        // getting 0.798 s on rel
        fib(34);

        debug::set(0, false);
        led::set(Color::Green);

        fib(33);

        let _stack_usage = stack::usage();

        #[cfg(feature = "exit")]
        {
            uart::write1( b"Stopping\r\n\0");
            hal::semihost::exit(0);
        }
    }
}

pub fn fib(x: usize) -> u32 {
    if x > 2 {
        fib(x - 1) + fib(x - 2)
    } else {
        1
    }
}
