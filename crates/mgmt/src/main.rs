//! Application main loop and entry point

#![no_std]
#![no_main]


extern crate bsp;
extern crate hal;


use bsp::console::Print;

use bsp::led;
use bsp::led::Color;

mod stack;
mod startup;


#[no_mangle]
#[export_name = "main"]
#[inline(never)]
/// Entry point for the application when the `std` feature is not enabled.
pub extern "C" fn main() -> ! {
    my_main();

    loop {}
}


#[inline(never)]
/// Main function that initializes the system and runs the task manager.
fn my_main() {
    //msg::test_msg();

    let mut bsp = bsp::BSP::new();

    bsp.init();
    
    //bsp.validate();


    b"Starting\r\n".print_console();
    

    led::set(Color::Green);

    let (stack_usage, stack_current, stack_reserved) = stack::usage(false);
    if true {
        b"  Starting stack usage: ".print_console();
        (stack_usage as u32).print_console();
        b" bytes\r\n".print_console();

        b"  Starting stack current: ".print_console();
        (stack_current as u32).print_console();
        b" bytes\r\n".print_console();

        b"  Starting stack reserved: ".print_console();
        (stack_reserved as u32).print_console();
        b" bytes\r\n".print_console();
    }

    loop {

    }
}
