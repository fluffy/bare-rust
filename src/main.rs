#![no_std]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

use hal;
use hal::led;
use hal::led::Color;

use hal::console::Print;

use hal::debug;

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

    b"Starting\r\n".print_console();

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
        b"  Duration: ".print_console();
        let duration_ms = (duration.as_u64()) / 1000; // convert to mS
        duration_ms.print_console();
        b" mS\r\n".print_console();

        led::set(Color::Blue);

        fib(32);

        let stack_usage = stack::usage() as u32;

        if cfg!(not(feature = "std")) {
            b"  Stack usage: ".print_console();
            stack_usage.print_console();
            b" bytes\r\n".print_console();
        }

        #[cfg(feature = "exit")]
        {
            b"Stopping\r\n".print_console();
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
