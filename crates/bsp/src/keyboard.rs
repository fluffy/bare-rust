extern crate hal;

#[cfg(feature = "std")]
extern crate std;

pub struct Keyboard {}

impl crate::keyboard::Keyboard {
    #[inline(never)]
    pub fn new() -> Self {
        crate::keyboard::Keyboard {}
    }

    #[inline(never)]
    pub fn init(&self) {}

    pub fn get_key(&self) -> u8 { 
        // returns 0 if no key has been pressed
        0
    }
}
