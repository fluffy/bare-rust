use core::ptr;

extern crate hal;

use hal::board;
use hal::cpu;

use hal::cpu::*;

pub struct Buttons {
    prev_ptt: bool,
    prev_ai: bool,
}

impl Buttons {
    #[inline(never)]
    pub fn new() -> Self {
        Buttons {
            prev_ptt: false,
            prev_ai: false,
        }
    }

    #[inline(never)]
    pub fn init(&mut self) {
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
    pub fn validate(&self) {

        if board::info::HAS_PTT_BUTTON {
            let pin_num: u32 = board::info::PTT_BUTTON.1 as u32;
            let gpio = board::info::PTT_BUTTON.0;

            if gpio != cpu::GPIO_C {
                panic!("PTT_BUTTON not on GPIO_C");
            }

            // Check if GPIO_C is enabled
            if hal::read!(RCC.ahb1enr[GPIOCEN;1]) != 1 {
                panic!("GPIO_C not enabled");
            }

            // Check if pin is set for input
            let moder = hal::read!(gpio.moder[ pin_num * 2 ; 2]);
            if moder != 0b00 {
                panic!("PTT button not set for input");
            }

            // Check if pin has pull-down enabled
            let pupdr = hal::read!(gpio.pupdr[ pin_num * 2 ; 2]);
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
            if hal::read!(RCC.ahb1enr[GPIOCEN;1]) != 1 {
                panic!("GPIO_C not enabled");
            }

            // Check if pin is set for input
            let moder = hal::read!(gpio.moder[ pin_num * 2 ; 2]);
            if moder != 0b00 {
                panic!("AI button not set for input");
            }

            // Check if pin has pull-down enabled
            let pupdr = hal::read!(gpio.pupdr[ pin_num * 2 ; 2]);
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

    pub fn read_ptt(&mut self) -> (bool, bool) {
        if board::info::HAS_PTT_BUTTON {
            let state =  board::info::PTT_BUTTON.read() != board::info::PTT_BUTTON_PULL_UP;
            let changed = state != self.prev_ptt;
            self.prev_ptt = state;
            return (state, changed);
        }
        (false, false)
    }
    
    pub fn read_ai(&mut self) -> (bool, bool) {
        if board::info::HAS_AI_BUTTON {
            let state =  board::info::AI_BUTTON.read() != board::info::AI_BUTTON_PULL_UP;
            let changed = state != self.prev_ai;
            self.prev_ai = state;
            return (state, changed);
        }
        (false, false)
    }
}
