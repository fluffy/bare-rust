#![no_std]
//! # HAL Crate
//!
//! This crate provides an HAL (Hardware Abstraction Layer) for the STM32F405RG
//! This crate is primary meant to be used by the bsp crate.
//! Most applications should use the bsp crate.
//!
//! Most of the information comes from the
//! [RM0090 Reference manual](https://www.st.com/resource/en/reference_manual/dm00031020-stm32f405-415-stm32f407-417-stm32f427-437-and-stm32f429-439-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf)
//!
//! ## Features
//!
//! To compile this crate you need to specify just one board as a feature.
//!
//! - `board-hactar12`: Enables support for the Hactar V12 board.
//! - `board-blinkA`: Enables support for the Blink Rev A board.
//! - `board-qemu`: Enables support for the QEMU emulator. BROKEN
//! - `board-sim`: Enables support for the simulation of board.
//!
//! For boards other than the simulation board, the target must be specified as well.
//! For example:
//! ```sh
//! cargo build --features=board-hactar12,bsp/std,app/std -target=thumbv7em-none-eabihf
//! ```
//!
//! ## Modules
//!
//! - `board`: Board-specific configurations and initializations.
//! - `clock`: Clock configuration and management.
//! - `cpu`: Function to access registers on the CPU
//! - `gpio`: General Purpose Input/Output (GPIO) management.
//! - `semihost`: Semihosting support.
//! - `svd`: Constants from the System View Description (SVD)
//! - `timer`: Timer configuration and management.
//! - `uart`: Serial port Receiver/Transmitter (UART) management.
//!
//! ## Usage
//!
//! ```rust
//!  use hal::cpu;
//!  use hal::gpio;
//!  
//!  let tx = gpio::Pin(cpu::GPIOA, 9);
//!  let rx = gpio::Pin(cpu::GPIOA, 10);
//!  let clock_freq = 16_000_000;
//!  hal::init(clock_freq, tx, rx);
//!
//!  let pin = gpio::Pin(cpu::GPIOA, 6);
//!  pin.output(); // set pin as output
//!  pin.high(); // set pin to logic level high
//!
//! ```
//!

//pub mod board;
pub mod clock;
pub mod cpu;
pub mod gpio;

pub mod timer;
pub mod uart;

pub mod svd_stm32f0x2;
pub mod svd_stm32f405;
pub mod watch_dog;

#[inline(never)]
/// Initializes the hardware.
pub fn init(hse_clk_freq: u32, tx_pin: gpio::Pin, rx_pin: gpio::Pin) {
    cpu::init();

    // always set up clocks first
    clock::init(hse_clk_freq);

    // Do after clock and memory is set up
    gpio::init();

    // do soon after clock is up so we  can use console
    uart::init1(115_200, tx_pin, rx_pin);
    // do after uart is up

    // Do last as this starts timer events
    #[cfg(feature = "stm32f405")]
    timer::init2();
}

#[inline(never)]
/// Validates the hardware has been correctly initialized.
pub fn validate() {
    #[cfg(not(feature = "std"))]
    clock::validate();
}
