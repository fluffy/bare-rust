use core::ptr;

use super::cpu::*;

#[derive(Copy, Clone)]
pub struct Pin(*mut GpioReg, u8);

macro_rules! write_2bits {
    ($x:ident.$y:ident, $bit_num:expr, $val:expr ) => {
        unsafe { priv_write_2bits(ptr::addr_of_mut!((*$x).$y), $bit_num, $val) }
    };
}
macro_rules! write_1bits {
    ($x:ident.$y:ident, $bit_num:expr, $val:expr ) => {
        unsafe { priv_write_1bits(ptr::addr_of_mut!((*$x).$y), $bit_num, $val) }
    };
}
macro_rules! write_reg {
    ($x:ident.$y:ident, $data:expr ) => {
        unsafe { priv_write_reg(ptr::addr_of_mut!((*$x).$y), $data) }
    };
}

macro_rules! read_1bits {
    ($x:ident.$y:ident, $bit_num:expr ) => {
        unsafe { priv_read_1bits(ptr::addr_of_mut!((*$x).$y), $bit_num) }
    };
}

impl Pin {
    pub fn new(gpio: *mut GpioReg, p: u8) -> Pin {
        assert!(p < 16);
        return Pin(gpio, p);
    }

    fn output(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set mode to output
        write_2bits!(gpio.moder, pin_num, 0b01);
        // set output as low
        write_1bits!(gpio.odr, pin_num, 0b0);
        // set as push-pull
        write_1bits!(gpio.otyper, pin_num, 0b0);
        // set no pull up , no pull down
        write_2bits!(gpio.pupdr, pin_num, 0b00);
        // set speed to slow
        write_2bits!(gpio.ospeedr, pin_num, 0b00);
    }

    fn input(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set to pull down
        write_2bits!(gpio.pupdr, pin_num, 0b10);
        // set mode to input
        write_2bits!(gpio.moder, pin_num, 0b00);
    }

    pub fn low(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        write_reg!(gpio.bsrr, 0b1 << (pin_num + 16));
        //write_1bits!(gpio.odr, pin_num, 0b0);
    }

    pub fn high(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        write_reg!(gpio.bsrr, 0b1 << (pin_num + 0));
        //write_1bits!(gpio.odr, pin_num, 0b1);
    }

    pub fn read(&self) -> bool {
        let gpio = self.0;
        let pin_num = self.1;

        return read_1bits!(gpio.idr, pin_num);
    }
}

#[cfg(feature = "brd-hactar-10")]
pub fn init() {
    Pin::new(GPIO_A, 6).output(); // red LED
    Pin::new(GPIO_C, 5).output(); // green LED
    Pin::new(GPIO_A, 1).output(); // blue LED

    Pin::new(GPIO_C, 0).input(); // PTT
}

#[cfg(feature = "brd-blink-clk-a")]
pub fn init() {
    Pin::new(GPIO_A, 12).output(); // red LED
    Pin::new(GPIO_A, 11).output(); // green LED
    Pin::new(GPIO_B, 7).output(); // blue LED

    Pin::new(GPIO_C, 13).input(); // PTT

    Pin::new(GPIO_A, 8).output(); // PPS Out
}
