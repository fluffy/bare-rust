use super::board;

pub fn init() {
    board::info::DEBUG1_PIN.low();
}

pub fn set(ch: u8, v: bool) {
    assert!(ch == 0);

    if v {
        board::info::DEBUG1_PIN.high();
    } else {
        board::info::DEBUG1_PIN.low();
    }
}
