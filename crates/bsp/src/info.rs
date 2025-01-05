extern crate hal;

#[cfg(feature = "std")]
extern crate std;

pub struct Info {}

impl crate::info::Info {
    #[inline(never)]
    pub fn new() -> Self {
        crate::info::Info {}
    }

    #[inline(never)]
    pub fn init(&self) {}

    pub fn get_device_id(&self) -> u64 {
        0x12345678
    }
    
    pub fn get_secret_key(&self) -> u128 {
        0x87654321
    }
    
    pub fn get_extra_random(&self) -> u128 {
        0x12345678
    }
    
    pub fn set_secret_key(&self, _key: u128) {
        // Set the secret key
    }
    
    pub fn get_hardware_revision(&self) -> u8 {
        12
    }
    
}
