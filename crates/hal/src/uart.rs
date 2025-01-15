//! # UART Module
//!
//! This module provides functionality for initializing and using the USART1 peripheral for serial communication.
//! It includes methods for setting up the USART1 with a specified baud rate and sending data through the USART1 interface.
//!
//! ## Functions
//!
//! - `init1`: Initializes the USART1 peripheral with the specified baud rate.
//! - `write1`: Sends a byte of data through the USART1 interface.
//!
//! ## Usage
//!
//! The `init1` function should be called during system startup to configure the USART1 peripheral. The `write1` function
//! can be used to send data through the USART1 interface.
//!
//! ## Example
//!
//! ```rust
//! use crate::hal::*;
//! use crate::hal::clock;
//!
//! fn main() {
//!     clock::init( 16_000_000 );
//!
//!     let tx = gpio::Pin(cpu::GPIO_A, 9);
//!     let rx = gpio::Pin(cpu::GPIO_A, 10);
//!     let baud_rate: u32 = 115200;
//!     uart::init1(115200,rx,tx);
//!
//!     // Send a byte of data
//!     uart::write1(b'H');
//!     uart::write1(b'i');
//! }
//! ```

use core::ptr;

#[cfg(feature = "std")]
extern crate std;

//use super::board;
use super::cpu;
use super::cpu::*;
use super::gpio;

pub use super::cpu::USART as USART1;

#[cfg(feature = "stm32f072")]
#[inline(never)]
pub fn init1(baud_rate: u64, tx_pin: gpio::Pin, rx_pin: gpio::Pin) {
    // Enable USART1 & GPIOA clock
    cpu::write!(RCC.apb2enr[USART1EN;1], 1);
    cpu::write!(RCC.ahbenr[IOPAEN;1], 1);

    // Configure PA9 (TX) and PA10 (RX) as alternate function (AF1 for USART1)
    assert!(tx_pin.0 == GPIO_A as *mut cpu::GpioReg);
    assert!(rx_pin.0 == GPIO_A as *mut cpu::GpioReg);

    let tx_pin = tx_pin.1;
    let rx_pin = rx_pin.1;

    cpu::write!(GPIO_A.moder[tx_pin*2;2], 0b10); // PA9 to AF mode
    cpu::write!(GPIO_A.moder[rx_pin*2;2], 0b10); // PA10 to AF mode
    cpu::write!(GPIO_A.afrh[(tx_pin-8)*4;4], 0b0001); // PA9 to AF1
    cpu::write!(GPIO_A.afrh[(rx_pin-8)*4;4], 0b0001); // PA10 to AF1

    // Set baud rate
    let apb_freq: u32 = 48_000_000; // APB clock frequency
    let usart_div: u32 = apb_freq / baud_rate as u32;
    cpu::write!(USART1.brr, usart_div);

    // Enable USART1, transmitter and receiver
    cpu::write!(USART1.cr1[UE;1], 1); // USART enable
    cpu::write!(USART1.cr1[TE;1], 1); // Transmitter enable
    cpu::write!(USART1.cr1[RE;1], 1); // Receiver enable
}

#[cfg(feature = "stm32f405")]
#[inline(never)]
pub fn init1(baud_rate: u64, tx_pin: gpio::Pin, rx_pin: gpio::Pin) {
    // enable USART1 & GPIO clock
    cpu::write!( RCC.apb2enr[USART1EN;1], 1);
    cpu::write!( RCC.ahb1enr[GPIOAEN;1], 1);

    // configure pins for USART1
    // AF7 work for USART1 to 3. afrh work pin 8 to 15
    assert!(tx_pin.0 == GPIO_A as *mut cpu::GpioReg);
    assert!(rx_pin.0 == GPIO_A as *mut cpu::GpioReg);

    let tx_pin = tx_pin.1;
    let rx_pin = rx_pin.1;

    cpu::write!( GPIO_A.moder[rx_pin*2;2], 0b10); // AF mode
    cpu::write!( GPIO_A.moder[tx_pin*2;2], 0b10); // AF mode

    if rx_pin < 8 {
        cpu::write!( GPIO_A.afrl[(rx_pin)*4;4], 7); // AF7 mode
    } else {
        cpu::write!( GPIO_A.afrh[(rx_pin-8)*4;4], 7); // AF7 mode
    }
    if tx_pin < 8 {
        cpu::write!( GPIO_A.afrl[(tx_pin)*4;4], 7); // AF7 mode
    } else {
        cpu::write!( GPIO_A.afrh[(tx_pin-8)*4;4], 7); // AF7 mode
    }

    // set baud rate
    // UART 1 is on APB2 bus, which is 84MHz
    let apb_freq: u64 = 84_000_000;
    let div_fixed3: u64 = 1000 * apb_freq / (16 * baud_rate);

    let mantissa: u64 = div_fixed3 / 1000;
    let frac: u64 = (div_fixed3 % 1000) * 16 / 1000;

    cpu::write!( USART1.brr[DIV_Mantissa;11], mantissa as u32);
    cpu::write!( USART1.brr[DIV_Fraction;4], frac as u32);

    cpu::write!( USART1.cr1[M;1], 0); // 8 data bits

    let even_parity = false;
    if even_parity {
        cpu::write!( USART1.cr1[PCE;1], 1); // parity control enable
        cpu::write!( USART1.cr1[PS;1], 0); // even parity
    } else {
        cpu::write!( USART1.cr1[PCE;1], 0); // no parity
    }
    cpu::write!( USART1.cr2[STOP;2], 0b00); // 1 stop bit

    cpu::write!( USART1.cr1[TE;1], 1); // transmit enable
    cpu::write!( USART1.cr1[RE;1], 1); // receive enable
    cpu::write!( USART1.cr1[UE;1], 1); // uart enable
}

#[cfg(feature = "stm32f405")]
pub fn write1(c: u8) {
    #[cfg(not(feature = "std"))]
    while (cpu::read!(USART1.sr[TXE;1]) == 0) {}
    cpu::write!(USART1.dr[DR;8], c as u32);
}

#[cfg(feature = "stm32f072")]
pub fn write1(c: u8) {
    // Wait until transmit data register is empty
    while cpu::read!(USART1.isr[TXE;1]) == 0 {}
    // Write the byte to the data register
    cpu::write!(USART1.tdr, c as u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock;

    #[test]
    fn test_uart() {
        clock::init(16_000_000);

        let tx = gpio::Pin(cpu::GPIO_A, 9);
        let rx = gpio::Pin(cpu::GPIO_A, 10);
        let baud_rate: u64 = 115200;

        init1(baud_rate, tx, rx);
        write1(b'O');
        write1(b'K');
    }
}
