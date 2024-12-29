use super::board;
use core::ptr;

use super::cpu;
use super::cpu::*;

#[inline(never)]
pub fn init() {
    board::info::PTT_BUTTON.input();
    board::info::PTT_BUTTON.pulldown();
}

#[inline(never)]
pub fn validate() {
    if cfg!(feature = "board-sim") {
        return;
    }

    // Check if GPIO_C is enabled
    if cpu::read!(RCC.ahb1enr[GPIOCEN;1]) != 1 {
        panic!("GPIO_C not enabled");
    }

    let pin_num: u32 = board::info::PTT_BUTTON.1 as u32;

    // Check if PC13 is set for input
    let moder = cpu::read!(GPIO_C.moder[ pin_num * 2 ; 2]);
    if moder != 0b00 {
        panic!("button not set for input");
    }

    // Check if PC13 has pull-down enabled
    let pupdr = cpu::read!(GPIO_C.pupdr[ pin_num * 2 ; 2]);
    if pupdr != 0b10 {
        panic!("button not set for pull-down");
    }
}

pub fn read_ptt() -> bool {
    board::info::PTT_BUTTON.read()
}
