extern crate hal;

#[cfg(feature = "std")]
extern crate std;

pub struct Battery {}

impl crate::battery::Battery {
    #[inline(never)]
    pub fn new() -> Self {
        crate::battery::Battery {}
    }

    #[inline(never)]
    pub fn init(&self) {}

    pub fn get_battery_percentage(&self) -> u8 {
        99
    }
}
