extern crate hal;

use hal::board;

#[cfg(feature = "std")]
extern crate std;

pub struct Led {}

impl Led {
    #[inline(never)]
    pub fn new() -> Self {
        Led {}
    }

    #[inline(never)]
    pub fn init(&self) {
        board::info::LED_RED_PIN.output();
        board::info::LED_GREEN_PIN.output();
        board::info::LED_BLUE_PIN.output();

        set(Color::Black);
    }
}

pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}
pub fn set(c: Color) {
    match c {
        Color::Red => {
            board::info::LED_RED_PIN.high();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
            if board::info::IS_SIM {
                #[cfg(feature = "std")]
                std::print!("LED: RED\r\n");
            }
        }
        Color::Green => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.high();
            board::info::LED_BLUE_PIN.low();
            if board::info::IS_SIM {
                #[cfg(feature = "std")]
                std::print!("LED: GREEN\r\n");
            }
        }
        Color::Blue => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.high();
            if board::info::IS_SIM {
                #[cfg(feature = "std")]
                std::print!("LED: BLUE\r\n");
            }
        }
        Color::Black => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
            if board::info::IS_SIM {
                #[cfg(feature = "std")]
                std::print!("LED: BLACK\r\n");
            }
        }
        Color::White => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
            if board::info::IS_SIM {
                #[cfg(feature = "std")]
                std::print!("LED: WHITE\r\n");
            }
        }
    }
}
