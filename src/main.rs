#![no_std]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

use hal;
use hal::led;
use hal::led::Color;
use hal::console;

use hal::{debug};

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

    #[cfg(feature = "exit")]
    hal::validate();

    console::write(b"Starting\r\n");
    
    // TODO remove 
    console::write(b"  junk: ");
    let junk:u64 = 1234;
    console::write_u64(junk);
    console::write(b" mS\r\n");
    
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
        console::write(b"  Duration: ");
        console::write_u64((duration.as_u64() ) / 1000); // convert to mS
        console::write(b" mS\r\n");

        led::set(Color::Blue);

        fib(32);

        let _stack_usage = stack::usage();

        #[cfg(feature = "exit")]
        {
            console::write(b"Stopping\r\n\0");
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
