#![no_std]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

extern crate dev;
extern crate hal;

use crate::channel::v_mpsc;
//use hal;
use dev::console::Print;
use dev::debug;
use dev::led;
use dev::led::Color;

//use crate::msg::Msg;

mod channel;
mod dispatch;
mod msg;
mod stack;
mod startup;
mod tasks;

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

#[inline(never)]
fn my_main() -> ! {
    //msg::test_msg();

    let mut bsp = dev::BSP::new();

    bsp.init();

    #[cfg(debug_assertions)]
    bsp.validate();

    b"Starting\r\n".print_console();

    let (mut sender, receiver): (v_mpsc::Sender<msg::Msg>, v_mpsc::Receiver<msg::Msg>) =
        v_mpsc::channel();

    let button_task = tasks::buttons_task::ButtonTask {};

    let mut task_mgr = tasks::TaskMgr::new(&mut sender, &mut bsp);
    task_mgr.add_task(&button_task);

    led::set(Color::Green);

    let stack_usage = stack::usage() as u32;

    if cfg!(not(feature = "std")) {
        b"  Starting stack usage: ".print_console();
        stack_usage.print_console();
        b" bytes\r\n".print_console();
    }

    fib_test();

    loop {
        task_mgr.run();
        dispatch::process(receiver);

        #[cfg(feature = "exit")]
        {
            b"Stopping\r\n".print_console();
            hal::semihost::exit(0);
        }
    }
}

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

pub fn fib(x: usize) -> u32 {
    if x > 2 {
        fib(x - 1) + fib(x - 2)
    } else {
        1
    }
}
