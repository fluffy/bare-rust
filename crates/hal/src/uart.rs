use core::ptr;

#[cfg(feature = "std")]
extern crate std;

use super::board;
use super::cpu;
use super::cpu::*;

pub use super::cpu::USART as USART1;

#[inline(never)]
pub fn init1(baud_rate: u64) {
    // enable USART1 & GPIO clock
    cpu::write!( RCC.apb2enr[USART1EN;1], 1);
    cpu::write!( RCC.ahb1enr[GPIOAEN;1], 1);

    // configure pins for USART1
    // AF7 work for USART1 to 3. afrh work pin 8 to 15
    assert!(board::info::CONSOLE_TX.0 == GPIO_A as *mut cpu::GpioReg);
    assert!(board::info::CONSOLE_RX.0 == GPIO_A as *mut cpu::GpioReg);

    let tx_pin = board::info::CONSOLE_TX.1;
    let rx_pin = board::info::CONSOLE_RX.1;

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
    cpu::write!( USART1.cr1[PCE;1], 0); // no parity
    cpu::write!( USART1.cr2[STOP;2], 0b00); // 1 stop bit

    cpu::write!( USART1.cr1[TE;1], 1); // transmit enable
    cpu::write!( USART1.cr1[RE;1], 1); // receive enable
    cpu::write!( USART1.cr1[UE;1], 1); // uart enable
}

pub fn write1(c: u8) {
    if board::info::IS_SIM {
        return;
    }
    while (cpu::read!(USART1.sr[TXE;1]) == 0) {}
    cpu::write!(USART1.dr[DR;8], c as u32);
}
