use core::ptr;

use super::cpu;
pub use super::cpu::*;
use super::gpio;

pub fn init1(
    spi_freq: u32,
    scl_pin: gpio::Pin,
    sda_pin: gpio::Pin,
) {
    scl_pin.output();
    sda_pin.output();
    
    // TODO 
    let _ = spi_freq;
    
    // enable clock for SPI1
    cpu::write!(RCC.apb2enr[SPI1EN;1], 1);
}
