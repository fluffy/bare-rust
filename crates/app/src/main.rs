//! Application main loop and entry point

#![no_std]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

extern crate dev;
extern crate hal;

use crate::channel::v_mpsc;
use dev::console::Print;

use dev::led;
use dev::led::Color;

mod channel;
mod dispatch;
mod fib;
mod metrics;
mod msg;
mod stack;
mod startup;
mod tasks;

#[cfg(not(feature = "std"))]
#[no_mangle]
#[export_name = "main"]
#[inline(never)]
/// Entry point for the application when the `std` feature is not enabled.
pub extern "C" fn main() -> ! {
    my_main();
}

#[cfg(feature = "std")]
/// Entry point for the application when the `std` feature is enabled.
fn main() -> () {
    my_main();
}

#[inline(never)]
/// Main function that initializes the system and runs the task manager.
fn my_main() -> ! {
    //msg::test_msg();

    let mut bsp = dev::BSP::new();

    bsp.init();

    #[cfg(debug_assertions)]
    bsp.validate();

    b"Starting\r\n".print_console();

    let (mut sender, receiver): (v_mpsc::Sender<msg::Msg>, v_mpsc::Receiver<msg::Msg>) =
        v_mpsc::channel();

    let mut metrics = metrics::Metrics::new();

    let mut task_mgr = tasks::TaskMgr::new(&mut sender, &mut bsp, &mut metrics);

    let button_task = tasks::buttons_task::ButtonTask {};
    task_mgr.add_task(&button_task);

    let metrics_task = tasks::metrics_task::MetricsTask {};
    task_mgr.add_task(&metrics_task);

    //let fib_task = tasks::fib_task::FibTask {};
    //task_mgr.add_task(&fib_task);

    led::set(Color::Green);

    let stack_usage = stack::usage(false) as u32;
    if cfg!(not(feature = "std")) {
        b"  Starting stack usage: ".print_console();
        stack_usage.print_console();
        b" bytes\r\n".print_console();
    }

    fib::fib_test();

    loop {
        task_mgr.run();
        dispatch::process(receiver);

        let now = hal::timer::current_time();

        if false {
            b"  now=".print_console();
            (now.as_u64() / 1000).print_console();
            b" mS\r\n".print_console();
        }

        #[cfg(feature = "exit")]
        {
            b"Stopping\r\n".print_console();
            hal::semihost::exit(0);
        }
    }
}
