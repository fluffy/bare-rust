use super::gpio;

pub fn init() {
    set(Color::Black);
}

pub enum Color {
    Black,
    Red,
    Green,
    Blue,
}
pub fn set(c: Color) {
    let red = gpio::Pin::new(gpio::GPIO_A, 6);
    let green = gpio::Pin::new(gpio::GPIO_C, 5);
    let blue = gpio::Pin::new(gpio::GPIO_A, 1);

    match c {
        Color::Red => {
            red.high();
            green.low();
            blue.low();
        }
        Color::Green => {
            red.low();
            green.high();
            blue.low();
        }
        Color::Blue => {
            red.low();
            green.low();
            blue.high();
        }
        Color::Black => {
            red.low();
            green.low();
            blue.low();
        }
    }
}
