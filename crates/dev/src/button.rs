use super::board;
use core::ptr;

use super::cpu;
use super::cpu::*;


#[inline(never)]
pub fn init() {
    if board::info::HAS_PTT_BUTTON {
        board::info::PTT_BUTTON.input();

        if board::info::PTT_BUTTON_PULL_UP {
            board::info::PTT_BUTTON.pullup();
        } else {
            board::info::PTT_BUTTON.pulldown();
        }
    }

    if board::info::HAS_AI_BUTTON {
        board::info::AI_BUTTON.input();

        if board::info::AI_BUTTON_PULL_UP {
            board::info::AI_BUTTON.pullup();
        } else {
            board::info::AI_BUTTON.pulldown();
        }
    }
}

#[inline(never)]
pub fn validate() {
    if cfg!(feature = "board-sim") {
        return;
    }

    if board::info::HAS_PTT_BUTTON {
        let pin_num: u32 = board::info::PTT_BUTTON.1 as u32;
        let gpio = board::info::PTT_BUTTON.0;

        if gpio != cpu::GPIO_C {
            panic!("PTT_BUTTON not on GPIO_C");
        }

        // Check if GPIO_C is enabled
        if cpu::read!(RCC.ahb1enr[GPIOCEN;1]) != 1 {
            panic!("GPIO_C not enabled");
        }

        // Check if pin is set for input
        let moder = cpu::read!(gpio.moder[ pin_num * 2 ; 2]);
        if moder != 0b00 {
            panic!("PTT button not set for input");
        }

        // Check if pin has pull-down enabled
        let pupdr = cpu::read!(gpio.pupdr[ pin_num * 2 ; 2]);
        if board::info::PTT_BUTTON_PULL_UP {
            if pupdr != 0b01 {
                panic!("PTT button not set for pull-up");
            }
        } else {
            if pupdr != 0b10 {
                panic!("PTT button not set for pull-down");
            }
        }
    }

    if board::info::HAS_AI_BUTTON {
        let pin_num: u32 = board::info::AI_BUTTON.1 as u32;
        let gpio = board::info::AI_BUTTON.0;

        if gpio != cpu::GPIO_C {
            panic!("AI_BUTTON not on GPIO_C");
        }

        // Check if GPIO_C is enabled
        if cpu::read!(RCC.ahb1enr[GPIOCEN;1]) != 1 {
            panic!("GPIO_C not enabled");
        }

        // Check if pin is set for input
        let moder = cpu::read!(gpio.moder[ pin_num * 2 ; 2]);
        if moder != 0b00 {
            panic!("AI button not set for input");
        }

        // Check if pin has pull-down enabled
        let pupdr = cpu::read!(gpio.pupdr[ pin_num * 2 ; 2]);
        if board::info::AI_BUTTON_PULL_UP {
            if pupdr != 0b01 {
                panic!("AI button not set for pull-up");
            }
        } else {
            if pupdr != 0b10 {
                panic!("AI button not set for pull-down");
            }
        }
    }
}

pub fn read_ptt() -> bool {
    if board::info::HAS_PTT_BUTTON {
        return board::info::PTT_BUTTON.read() != board::info::PTT_BUTTON_PULL_UP;
    }
    false
}
