use super::cpu;
use super::gpio;
use super::board;

pub fn init() {
        board::info::DEBUG1_PIN.low();
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
    #[cfg(feature = "board-hactar12")]
    {
        assert!(ch == 0);
        // not the PPS output is inverted
        if v {
            gpio::Pin::new(cpu::GPIO_A, 11).low(); // PPS line
        } else {
            gpio::Pin::new(cpu::GPIO_A, 11).high(); // PPS line
        }
    }    
    #[cfg(feature = "board-sim")]
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
