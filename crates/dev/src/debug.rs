extern crate hal;

use hal::board;

pub struct Debug {}


impl Debug {
    #[inline(never)]
    pub fn new() -> Self {
        Debug {}
    }

    #[inline(never)]
    pub fn init(&self) {
        board::info::DEBUG1_PIN.output();
        board::info::DEBUG1_PIN.low();
    }
}


pub fn set(ch: u8, v: bool) {
    assert!(ch == 0);

    if v {
        board::info::DEBUG1_PIN.high();
    } else {
        board::info::DEBUG1_PIN.low();
    }
}
