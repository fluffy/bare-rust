use super::cpu;
use super::gpio;

pub fn init() {
    #[cfg(feature = "board-blinkA")]
    {
        gpio::Pin::new(cpu::GPIO_A, 8).low(); // PPS line
    }
    #[cfg(feature = "board-hactar10")]
    {
        gpio::Pin::new(cpu::GPIO_A, 11).low(); // UI DEBUG 1 
    }
}

pub fn set(ch: u8, v: bool) {
    #[cfg(feature = "board-blinkA")]
    {
        assert!(ch == 0);
        // not the PPS output is inverted
        if v {
            gpio::Pin::new(cpu::GPIO_A, 8).low(); // PPS line
        } else {
            gpio::Pin::new(cpu::GPIO_A, 8).high(); // PPS line
        }
    }
    #[cfg(feature = "board-hactar10")]
    {
        assert!(ch == 0);
        // not the PPS output is inverted
        if v {
            gpio::Pin::new(cpu::GPIO_A, 11).low(); // PPS line
        } else {
            gpio::Pin::new(cpu::GPIO_A, 11).high(); // PPS line
        }
    }
}
