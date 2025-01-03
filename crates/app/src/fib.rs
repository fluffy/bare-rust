//! A simple Fibonacci function.

#[cfg(feature = "std")]
extern crate std;

extern crate dev;

use dev::console::Print;
use dev::debug;
use dev::led;
use dev::led::Color;

#[inline(never)]
/// Function to test performance using the Fibonacci calculation.
pub fn fib_test() {
    // fib*34) getting 1.630 s on dev
    // fib(34) getting 0.798 s on rel. Now getting 764 mS - no idea what changed

    dev::led::set(Color::Blue);

    debug::set(0, true);
    let start_time = hal::timer::current_time();
    fib(34);
    let end_time = hal::timer::current_time();
    debug::set(0, false);

    led::set(Color::Green);

    let duration = end_time.sub(start_time);
    b"  Duration fib(34): ".print_console();
    let duration_ms = (duration.as_u64()) / 1000; // convert to mS
    duration_ms.print_console();
    b" mS\r\n".print_console();
}

#[inline(never)]
/// calculate the x'th Fibonacci number
pub fn fib(x: usize) -> u32 {
    if x > 2 {
        fib(x - 1) + fib(x - 2)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_sanity_check() {
        let result = fib(6);
        assert_eq!(result, 8);
    }

}
