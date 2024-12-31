#[cfg(not(any(
    feature = "board-hactar12",
    feature = "board-blinkA",
    feature = "board-qemu",
    feature = "board-sim"
)))]
compile_error!("Must specify a board as compile feature. Try --features=board-sim");

#[cfg(all(feature = "board-hactar12", feature = "board-sim"))]
compile_error!(
    "Must specify only a single board as a feature. Try --no-default-features --features=board-sim"
);

#[cfg(all(feature = "board-blinkA", feature = "board-sim"))]
compile_error!(
    "Must specify only a single board as a feature. Try --no-default-features --features=board-sim"
);

#[cfg(all(feature = "board-qemu", feature = "board-sim"))]
compile_error!(
    "Must specify only a single board as a feature. Try --no-default-features --features=board-sim"
);

#[cfg(feature = "board-hactar12")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const HAS_RCC: bool = true;
    pub const IS_SIM: bool = false;
    
    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_C, 5);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 1);

    pub const HAS_PTT_BUTTON: bool = true;
    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 0);
    pub const PTT_BUTTON_PULL_UP: bool = true;

    pub const HAS_AI_BUTTON: bool = true;
    pub const AI_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 1);
    pub const AI_BUTTON_PULL_UP: bool = true;

    pub const CONSOLE_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 9);
    pub const CONSOLE_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 10);
}

#[cfg(feature = "board-blinkA")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const HAS_RCC: bool = true;
    pub const IS_SIM: bool = false;

    pub const CLOCK_PLL_M: u32 = 8;

    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 8);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 12);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_B, 7);

    pub const HAS_PTT_BUTTON: bool = true;
    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 13);
    pub const PTT_BUTTON_PULL_UP: bool = false;

    pub const HAS_AI_BUTTON: bool = false;
    pub const AI_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 13);
    pub const AI_BUTTON_PULL_UP: bool = false;

    pub const CONSOLE_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 9);
    pub const CONSOLE_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 10);
}

#[cfg(feature = "board-qemu")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const HAS_RCC: bool = false;
    pub const IS_SIM: bool = false;

    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_C, 5);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 1);

    pub const HAS_PTT_BUTTON: bool = false;
    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 0);
    pub const PTT_BUTTON_PULL_UP: bool = true;

    pub const HAS_AI_BUTTON: bool = false;
    pub const AI_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 1);
    pub const AI_BUTTON_PULL_UP: bool = true;

    pub const CONSOLE_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 9);
    pub const CONSOLE_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 10);
}

#[cfg(feature = "board-sim")]
pub mod info {
    use crate::cpu;
    use crate::gpio;

    pub const HAS_RCC: bool = false;
    pub const IS_SIM: bool = true;

    pub const CLOCK_PLL_M: u32 = 12;
    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_C, 5);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 1);

    pub const HAS_PTT_BUTTON: bool = false;
    pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 0);
    pub const PTT_BUTTON_PULL_UP: bool = false;

    pub const HAS_AI_BUTTON: bool = false;
    pub const AI_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 1);
    pub const AI_BUTTON_PULL_UP: bool = false;

    pub const CONSOLE_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 9);
    pub const CONSOLE_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 10);
}
