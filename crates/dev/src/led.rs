extern crate hal;

use hal::board;

#[cfg(feature = "std")]
extern crate std;

#[inline(never)]
pub fn init() {
    board::info::LED_RED_PIN.output();
    board::info::LED_GREEN_PIN.output();
    board::info::LED_BLUE_PIN.output();

    set(Color::Black);
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
            #[cfg(feature = "std")]
            if cfg!(feature = "board-sim") {
                std::print!("LED: RED\r\n");
            }
        }
        Color::Green => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.high();
            board::info::LED_BLUE_PIN.low();
            #[cfg(feature = "std")]
            if cfg!(feature = "board-sim") {
                std::print!("LED: GREEN\r\n");
            }
        }
        Color::Blue => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.high();
            #[cfg(feature = "std")]
            if cfg!(feature = "board-sim") {
                std::print!("LED: BLUE\r\n");
            }
        }
        Color::Black => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
            #[cfg(feature = "std")]
            if cfg!(feature = "board-sim") {
                std::print!("LED: BLACK\r\n");
            }
        }
        Color::White => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
            #[cfg(feature = "std")]
            if cfg!(feature = "board-sim") {
                std::print!("LED: WHITE\r\n");
            }
        }
    }
}
