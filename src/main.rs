#![cfg_attr(  target_arch = "arm", no_std )]
#![cfg_attr(  target_arch = "arm", no_main )]

use hal;
use hal::debug;
use hal::led;
use hal::led::Color;

mod startup;
//mod hal;

#[cfg(target_arch = "arm")]
use core::panic::PanicInfo;
#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

//#[cfg(not(target_arch = "arm"))]
//use core::panic::PanicInfo;
//#[cfg(not(target_arch = "arm"))]
//#[panic_handler]
//fn panic(_panic: &PanicInfo) -> ! {
//    loop {}
//}


#[cfg(target_arch = "arm")]
#[no_mangle]
//#[start]
#[export_name = "main"]
#[inline(never)]
pub extern "C" fn main() -> ! {
    my_main();
}



#[cfg(not(target_arch = "arm"))]
//#[start]
//#[export_name = "main"]
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
