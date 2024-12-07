use super::gpio;

pub fn init() {
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
    let red: gpio::Pin;
    let green: gpio::Pin;
    let blue: gpio::Pin;

    #[cfg(feature = "brd-hactar-10")]
    {
        red = gpio::Pin::new(gpio::GPIO_A, 6);
        green = gpio::Pin::new(gpio::GPIO_C, 5);
        blue = gpio::Pin::new(gpio::GPIO_A, 1);
    }
    #[cfg(feature = "brd-blink-clk-a")]
    {
        red = gpio::Pin::new(gpio::GPIO_A, 12); // red LED
        green = gpio::Pin::new(gpio::GPIO_A, 11); // green LED
        blue = gpio::Pin::new(gpio::GPIO_B, 7); // blue LED
    }

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
        Color::White => {
            red.low();
            green.low();
            blue.low();
        }
    }
}
