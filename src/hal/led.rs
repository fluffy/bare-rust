use super::board;

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
        }
        Color::Green => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.high();
            board::info::LED_BLUE_PIN.low();
        }
        Color::Blue => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.high();
        }
        Color::Black => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
        }
        Color::White => {
            board::info::LED_RED_PIN.low();
            board::info::LED_GREEN_PIN.low();
            board::info::LED_BLUE_PIN.low();
        }
    }
}
