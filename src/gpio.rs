// use super::hal;

pub fn init() {}

    pub enum Bank {
        A,
        B,
        C
    }

    pub struct Pin(Bank, u8);

    impl Pin {
        pub fn new(b: Bank, p: u8) -> Pin {
            return Pin(b, p);
        }

        pub fn high(&self) { }
        pub fn low(&self) {}
    }

