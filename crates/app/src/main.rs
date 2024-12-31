#![no_std]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(feature = "std")]
extern crate std;

extern crate dev;
extern crate hal;

use crate::msg::v_mpsc;
//use hal;
use dev::console::Print;
use dev::debug;
use dev::led;
use dev::led::Color;

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

fn dispatch(msg: msg::Msg) {
    match msg {
        msg::Msg::PttButton(pressed) => {
            if pressed {
                b"  PTT button pressed\r\n".print_console();
            } else {
                b"  PTT button released\r\n".print_console();
            }
        }
        _ => {}
    }
}

#[inline(never)]
fn my_main() -> ! {
    //msg::test_msg();

    hal::init();

    //#[cfg(feature = "exit")]
    hal::validate();

    b"Starting\r\n".print_console();

    let (sender, receiver) = v_mpsc::channel();

    let button_task = tasks::ButtonTask { prev_state: false };

    let mut task_mgr = tasks::TaskMgr::new(sender);
    task_mgr.add_task(&button_task);

    loop {
        led::set(Color::Green);

        task_mgr.run();

        loop {
            let msg = receiver.recv();
            if msg == msg::Msg::None {
                break;
            }
            dispatch(msg);
        }

        // fib*34) getting 1.630 s on dev
        // fib(34) getting 0.798 s on rel. Now getting 764 mS - no idea what changed
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

        let bool = dev::button::read_ptt();
        if bool {
            b"  PTT button pressed\r\n".print_console();
        }

        dev::led::set(Color::Blue);

        fib(32);

        let stack_usage = stack::usage() as u32;

        if cfg!(not(feature = "std")) {
            b"  Stack usage: ".print_console();
            stack_usage.print_console();
            b" bytes\r\n".print_console();
        }

        b"\r\n".print_console();

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
