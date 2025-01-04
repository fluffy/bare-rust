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
//!  hal::init(); //
//!
//!  let pin = gpio::Pin(cpu::GPIO_A, 6);
//!  pin.output(); // set pin as output
//!  pin.high(); // set pin to logic level high
//!
//! ```
//!

pub mod board;
pub mod clock;
pub mod cpu;
pub mod gpio;
pub mod semihost;
pub mod svd;
pub mod timer;
pub mod uart;

#[inline(never)]
/// Initializes the hardware.
pub fn init() {
    cpu::init();

    // always set up clocks first
    clock::init();

    // Do after clock and memory is set up
    gpio::init();

    // do soon after clock is up so we  can use console
    uart::init1(115_200);
    // do after uart is up

    // Do last as this starts timer events
    timer::init2();
}

#[inline(never)]
/// Validates the hardware has been correctly initialized.
pub fn validate() {
    clock::validate();
}
