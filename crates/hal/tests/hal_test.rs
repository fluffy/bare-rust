#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests {
    //use hal::cpu::GPIOA;
    use hal::cpu;
    use hal::gpio;

    #[test]
    fn test_init() {
        let tx = gpio::Pin(cpu::GPIOA, 9);
        let rx = gpio::Pin(cpu::GPIOA, 10);
        let freq = 16_000_000;

        hal::init(freq, tx, rx);

        hal::validate();
        //assert!(false);
    }
}
