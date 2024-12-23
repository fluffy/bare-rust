use super::cpu;
use super::gpio;

pub fn init() {
    #[cfg(feature = "board-blinkA")]
    {
        gpio::Pin::new(cpu::GPIO_A, 8).low(); // PPS line
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
}
