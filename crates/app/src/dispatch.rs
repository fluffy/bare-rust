extern crate dev;

#[cfg(feature = "std")]
extern crate std;

use crate::channel::v_mpsc;
use crate::msg;
use dev::console::Print;

pub fn process(receiver: v_mpsc::Receiver<msg::Msg>) {
    loop {
        let msg = receiver.recv();
        if msg == msg::Msg::None {
            break;
        }
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
}
