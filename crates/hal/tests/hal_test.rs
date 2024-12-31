#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests {
    #[test]
    fn test_init() {
        hal::init();
        hal::validate();
        //assert!(false);
    }
}
