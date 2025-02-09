use core::ptr;

use super::cpu;
pub use super::cpu::*;
use super::gpio;

#[cfg(feature = "stm32f072")]
#[inline(never)]
pub fn init1(spi_freq: u32, scl_pin: gpio::Pin, sda_pin: gpio::Pin) {
    let _ = (spi_freq, scl_pin, sda_pin);
    assert!(false);
}

#[cfg(feature = "stm32f405")]
#[inline(never)]
pub fn init1(spi_freq: u32, scl_pin: gpio::Pin, sda_pin: gpio::Pin) {
    scl_pin.output();
    sda_pin.output();

    // TODO
    assert!(spi_freq >= 48_000_000 / 8);

    // enable clock for SPI1
    cpu::write!(RCC.apb2enr[SPI1EN;1], 0b1);

    // set up pins - AF from Figur 26 of RM0090 Reference Manual
    scl_pin.alt_fun(5, true); // AF5 for SPI1
    sda_pin.alt_fun(5, true); // AF5 for SPI1

    // set up the SPI
    cpu::write!(SPI1.cr1[BIDIMODE;1], 0b1); // set BIDIMODE to 1 line brdirections
    cpu::write!(SPI1.cr1[BIDIMODE;1], 0b1); // set BIDIOE to output
    cpu::write!(SPI1.cr1[DFF;1], 0b0); // set to 8 bit frame
    cpu::write!(SPI1.cr1[LSBFIRST;1], 0b0); //set LSB send first
    cpu::write!(SPI1.cr1[RXONLY;1], 0b0); // set to full duplex

    cpu::write!( SPI1.cr1[BR;3] , 0b010 ); // set baud rate to 1/8

    cpu::write!( SPI1.cr1[MSTR;1] , 0b1 ); // set to master mode
    cpu::write!( SPI1.cr1[CPOL;1] , 0b1 ); // set clock polarity
    cpu::write!( SPI1.cr1[CPHA;1] , 0b0 ); // set to 0

    cpu::write!( SPI1.cr1[SPE;1] , 0b1 ); // enable SPI

    // sends some data to the SPI
}
