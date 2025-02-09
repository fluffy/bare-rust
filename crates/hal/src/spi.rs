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

    sda_pin.pullup();


    // TODO
    assert!(spi_freq >= 48_000_000 / 8);

    // enable clock for SPI1
    cpu::write!(RCC.apb2enr[SPI1EN;1], 0b1);

    // set up pins - AF from Figur 26 of RM0090 Reference Manual
    // also see table 9 "Alternate function mapping" table of the datasheet stm32f405zg.pdf
    scl_pin.alt_fun(5, true); // AF5 for PA5 is SPI1_SCK
    sda_pin.alt_fun(5, true); // AF5 for PA7 is SPI1_MOSI
    assert!(scl_pin.0 == gpio::GPIOA);
    assert!(sda_pin.0 == gpio::GPIOA);
    assert!(scl_pin.1 == 5);
    assert!(sda_pin.1 == 7);

    // set up the SPI
    cpu::write!(SPI1.cr1[DFF;1], 0b0); // set to 8 bit frame
    cpu::write!(SPI1.cr1[LSBFIRST;1], 0b0); // set LSB send first
    cpu::write!(SPI1.cr1[RXONLY;1], 0b0); // set to full duplex
    cpu::write!(SPI1.cr1[CRCEN;1], 0b0); // disable CRC
    cpu::write!( SPI1.cr1[BR;3] , 0b100 ); // TODO set baud rate to 1/8 (0b010 )

    cpu::write!( SPI1.cr1[MSTR;1] , 0b1 ); // set to master mode
    cpu::write!(SPI1.cr1[BIDIMODE;1], 0b1); // set BIDIMODE to 1 line both directions
    cpu::write!(SPI1.cr1[BIDIOE;1], 0b1); // set BIDIOE to output

    cpu::write!( SPI1.cr1[SSM;1] , 0b1 );
    cpu::write!( SPI1.cr1[SSI;1] , 0b1 );

    cpu::write!( SPI1.cr1[CPOL;1] , 0b0 ); // low when idle
    cpu::write!( SPI1.cr1[CPHA;1] , 0b0 ); // sample on rising edge

    cpu::write!( SPI1.cr1[SPE;1] , 0b1 ); // do ot enable SPI yet

    if true {
        // wait for SPI to not be busy
        while cpu::read!(SPI1.sr[BSY;1]) != 0 {
            super::uart::write1(b'^');
        }

        // make sure TX buffer is empty
        while cpu::read!(SPI1.sr[TXE;1]) == 0 {
            super::uart::write1(b'V');
        }
        
        super::uart::write1(b'=');
        // sends some data to the SPI
        //cpu::write!(SPI1.cr1[BIDIMODE;1], 0b1); // set BIDIOE to output

        loop {
            super::uart::write1(b'>');

            cpu::write!( SPI1.cr1[SPE;1] , 0b1 ); // enable SPI
            
            cpu::write!(SPI1.dr, 0x59); // send some data

            // wait for transmit buffer to be empty
            while cpu::read!(SPI1.sr[TXE;1]) == 0 {
                super::uart::write1(b'.');
             }

            cpu::write!( SPI1.cr1[SPE;1] , 0b0 ); // disble  SPI 
            super::uart::write1(b'*');
        }

        //while cpu::read!(SPI1.sr[RXNE;1]) == 0 {} // wait for receive buffer to be full
        //let _ = cpu::read!(SPI1.dr[DR;8]); // read the data
    }
}
