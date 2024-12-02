#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod startup;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[export_name = "main"]
pub extern "C" fn main() -> ! {
    loop {}
}
