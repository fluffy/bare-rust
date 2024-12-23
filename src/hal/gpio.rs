use core::ptr;
use super::{board, cpu};


#[derive(Copy, Clone)]
pub struct Pin(pub *mut cpu::GpioReg, pub u8);

impl Pin {
    pub fn new(gpio: *mut cpu::GpioReg, p: u8) -> Pin {
        assert!(p < 16);
        return Pin(gpio, p);
    }

    fn output(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set mode to output
        cpu::write!( gpio.moder[pin_num*2;2], 0b01);

        // set output as low
        cpu::write!( gpio.odr[pin_num*1;1], 0b0);

        // set as push-pull
        cpu::write!( gpio.otyper[pin_num*1;1], 0b0);

        // set no pull up , no pull down
        cpu::write!( gpio.pupdr[pin_num*2;2], 0b00);

        // set speed to slow
        cpu::write!( gpio.ospeedr[pin_num*2;2], 0b00);
    }

    fn input(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set to pull down
        cpu::write!( gpio.pupdr[pin_num*2;2], 0b10);

        // set mode to input
        cpu::write!( gpio.moder[pin_num*2;2], 0b00);
    }

    pub fn low(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        cpu::write!(gpio.bsrr, 0b1 << (pin_num + 16));
    }

    pub fn high(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        cpu::write!(gpio.bsrr, 0b1 << (pin_num + 0));
    }

    pub fn read(&self) -> bool {
        let gpio = self.0;
        let pin_num: u32 = self.1 as u32;

        let val = cpu::read!( gpio.idr[pin_num*1;1] );
        val != 0
    }
}


pub fn init() {
    board::info::LED_RED_PIN.output(); 
    board::info::LED_GREEN_PIN.output();
    board::info::LED_BLUE_PIN.output();

    board::info::PTT_BUTTON.input();

    board::info::DEBUG1_PIN.output();
}

