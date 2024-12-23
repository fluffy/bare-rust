#[cfg(feature = "board-hactar12")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);
}

#[cfg(feature = "board-sim")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);
}

#[cfg(feature = "board-blinkA")]
pub mod info {
    use crate::cpu;
    use crate::gpio;
    
    pub const CLOCK_PLL_M: u32 = 8;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 8);
}
