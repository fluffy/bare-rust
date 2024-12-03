//use super::hal;

//use super::hal::gpio;
//use gpio;


    use super::gpio;

    pub fn init() {}

    pub enum Color {
        Red,
        Green,
        Blue,
    }
    pub fn set(c: Color) {
        /*
        PA6 is red
        PC5 is green
        PA1 is blue
         */
        let red = gpio::Pin::new( gpio::Bank::A, 6);

        match c {
            Color::Red => { red.high(); }
            Color::Green => { red.low(); }
            Color::Blue => { red.low(); }
        }
    }
