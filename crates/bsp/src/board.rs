//! # Board Module
//!
//! This module provides board-specific configurations and initializations for different supported boards.
//! It ensures that only one board is specified as a feature during compilation and provides constants
//! and configurations specific to each board.
//!
//! ## Supported Boards
//!
//! - `board-hactar12`: Configuration for the Hactar V12 board.
//! - `board-blinkA`: Configuration for the Blink Rev A board.
//! - `board-qemu`: Configuration for the QEMU emulator.
//! - `board-sim`: Configuration for the simulation of the board.
//!
//! ## Usage
//!
//! To compile this module, you need to specify one of the supported boards as a feature. For example:
//! ```sh
//! cargo build --features=board-hactar12 -target=thumbv7em-none-eabihf
//! ```
//!
//! ## Board Information
//!
//! Each board configuration provides the following information:
//!
//! - `HAS_RCC`: Indicates if the board has an RCC (Reset and Clock Control).
//! - `IS_SIM`: Indicates if the board is a simulation.
//! - `CLOCK_PLL_M`: The PLL (Phase-Locked Loop) multiplier for the clock.
//! - `DEBUG1_PIN`: The pin used for debug signal output.
//! - `LED_RED_PIN`, `LED_GREEN_PIN`, `LED_BLUE_PIN`: Pins for the red, green, and blue LEDs.
//! - `HAS_PTT_BUTTON`: Indicates if the board has a PTT (Push-To-Talk) button.
//! - `PTT_BUTTON`: The pin for the PTT button.
//! - `PTT_BUTTON_PULL_UP`: Indicates if the PTT button has a pull-up resistor.
//! - `HAS_AI_BUTTON`: Indicates if the board has an AI (Artificial Intelligence) button.
//! - `AI_BUTTON`: The pin for the AI button.
//! - `AI_BUTTON_PULL_UP`: Indicates if the AI button has a pull-up resistor.
//! - `CONSOLE_TX`, `CONSOLE_RX`: Pins for the console serial TX and RX.
//!

#[cfg(not(any(
    feature = "board-hactar12-mgmt",
    feature = "board-hactar12",
    feature = "board-blinkA",
    feature = "board-qemu",
    feature = "board-sim"
)))]
compile_error!(
    "Must specify a board as compile feature. Try --features=hal/board-sim,bsp/std,app/std"
);

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
    use hal::cpu;
    use hal::gpio;

    pub const CLOCK_HSE_FREQ: u32 = 24_000_000; // set to 0 for simulation

    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LEDS_INVERTED: bool = true;
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

#[cfg(feature = "board-hactar12-mgmt")]
pub mod info {
    use hal::cpu;
    use hal::gpio;

    pub const CLOCK_HSE_FREQ: u32 = 16_000_000; // set to 0 for simulation

    pub const NUM_DEBUG_PINS: usize = 0;
    //pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LEDS_INVERTED: bool = true;
    pub const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 4);
    pub const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    pub const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 7);

    pub const HAS_PTT_BUTTON: bool = false;
    //pub const PTT_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 0);
    //pub const PTT_BUTTON_PULL_UP: bool = true;

    pub const HAS_AI_BUTTON: bool = false;
    //pub const AI_BUTTON: gpio::Pin = gpio::Pin(cpu::GPIO_C, 1);
    //pub const AI_BUTTON_PULL_UP: bool = true;

    pub const CONSOLE_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 9);
    pub const CONSOLE_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 10);

    pub const UI_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 2);
    pub const UI_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 3);

    pub const UI_BOOT0: gpio::Pin = gpio::Pin(cpu::GPIO_A, 15);
    pub const UI_NRST: gpio::Pin = gpio::Pin(cpu::GPIO_B, 3);

    pub const NET_BOOT0: gpio::Pin = gpio::Pin(cpu::GPIO_B, 5);
    pub const NET_NRST: gpio::Pin = gpio::Pin(cpu::GPIO_B, 4);

    pub const MCLK: gpio::Pin = gpio::Pin(cpu::GPIO_A, 8);
    pub const MCLK_FREQ: u32 = 24_000_000;
}

#[cfg(feature = "board-blinkA")]
pub mod info {
    use hal::cpu;
    use hal::gpio;

    pub const CLOCK_HSE_FREQ: u32 = 16_000_000; // set to 0 for simulation

    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 8);

    pub const LEDS_INVERTED: bool = false;
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
    use hal::cpu;
    use hal::gpio;

    pub const CLOCK_HSE_FREQ: u32 = 16_000_000; // set to 0 for simulation

    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LEDS_INVERTED: bool = false;
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
    use hal::cpu;
    use hal::gpio;

    pub const CLOCK_HSE_FREQ: u32 = 0_000_000; // set to 0 for simulation

    pub const DEBUG1_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 11);

    pub const LEDS_INVERTED: bool = false;
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_board() {
        // make sure this file shows up in test coverage
        let freq = board::info::CLOCK_HSE_FREQ;
        assert_ne!(freq, 1);
    }
}
