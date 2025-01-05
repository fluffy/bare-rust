//!
//! This module is responsible for processing all the incoming messages
//! from tasks and dispatching them to the appropriate task for handling.
//!

extern crate bsp;

#[cfg(feature = "std")]
extern crate std;

use crate::channel::mpsc;
use crate::{msg, tasks};
use bsp::console::Print;

/// Processes all the incoming messages from tasks and dispatches them to the
/// appropriate task for handling.
///
/// Consumes all the messages from the receiver and
/// processes them each time it is called.
///
pub fn process(receiver: mpsc::Receiver<msg::Msg>, task_mgr: &mut tasks::TaskMgr) {
    let mut loop_count = 0;
    loop {
        let msg = receiver.recv();
        if msg == msg::Msg::None {
            break;
        }
        match msg {
            msg::Msg::PttButton(pressed) => {
                let _ = pressed;
                b"  PTT button dispatche\r\n".print_console();
            }
            msg::Msg::Keyboard { key } => {
                b"  Keyboard key press: ".print_console();
                (key as u32).print_console();
                b" dispatched\r\n".print_console();

                tasks::text_edit_task::recv(
                    &msg,
                    &mut task_mgr.sender,
                    &mut task_mgr.bsp,
                    &mut task_mgr.data,
                    &mut task_mgr.metrics,
                );
            }
            msg::Msg::TextInput {  input } => {
                let _ = input;
                b"  Text input dispatched\r\n".print_console();
            }
            _ => {}
        }

        loop_count += 1;
        if loop_count > 10 {
            break;
        }
    }
}
