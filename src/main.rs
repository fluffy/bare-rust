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
        led::set(Color::Green);

        // fib*34) getting 1.630 s on dev
        // fib(34) getting 0.798 s on rel
        debug::set(0, true);
        let start_time = hal::timer::current_time();
        fib(34);
        let end_time = hal::timer::current_time();
        debug::set(0, false);

        let duration = end_time.sub(start_time);
        uart::write1(b"  Duration: \0");
        uart::write1_u64((duration.as_u64() ) / 1000); // convert to mS
        uart::write1(b" mS\r\n\0");

        led::set(Color::Blue);

        fib(32);

        let _stack_usage = stack::usage();

        #[cfg(feature = "exit")]
        {
            uart::write1(b"Stopping\r\n\0");
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
