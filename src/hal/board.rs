#[cfg(not(any(
    feature = "board-hactar12",
    feature = "board-blinkA",
    feature = "board-sim"
)))]
compile_error!("Must specify a board as compile feature. Try--features=board-sim");

#[cfg(feature = "board-hactar12")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_C, 5);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 1);

    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 0);
}

#[cfg(feature = "board-sim")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_C, 5);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 1);

    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 0);
}

#[cfg(feature = "board-blinkA")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const CLOCK_PLL_M: u32 = 8;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 8);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 12);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_B, 7);

    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 13);
}
