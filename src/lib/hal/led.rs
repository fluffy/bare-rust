

use hal;
use hal::gpio;

mod hal {
    pub mod led {
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
            let red = gpio::Pin(A, 6);

            match c {
                Color::Red => {}
                Color::Green => {}
                Color::Blue => {}
                _ => { todo!(); }
            }
        }
    }
}