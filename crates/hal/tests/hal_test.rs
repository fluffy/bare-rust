#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests {
    //use hal::cpu::GPIO_A;
    use hal::cpu;
    use hal::gpio;

    #[test]
    fn test_init() {
        let tx = gpio::Pin(cpu::GPIO_A, 9);
        let rx = gpio::Pin(cpu::GPIO_A, 10);
        let freq = 16_000_000;

        hal::init(freq, tx, rx);

        hal::validate();
        //assert!(false);
    }
}
