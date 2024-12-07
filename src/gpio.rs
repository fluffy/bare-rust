use core::ptr;

#[derive(Copy, Clone)]
pub struct Pin(*mut GpioReg, u8);

#[repr(C)]
pub struct GpioReg {
    moder: u32,
    otyper: u32,
    ospeedr: u32,
    pupdr: u32,
    idr: u32,
    odr: u32,
    bsrr: u32,
    lckr: u32,
    afrl: u32,
    afrh: u32,
    brr: u32,
}

pub const GPIO_A: *mut GpioReg = 0x4002_0000 as *mut GpioReg;
pub const GPIO_B: *mut GpioReg = 0x4002_0400 as *mut GpioReg;
pub const GPIO_C: *mut GpioReg = 0x4002_0800 as *mut GpioReg;

fn priv_write_2bits(addr: *mut u32, num: u8, data: u32) {
    assert!(num < 16);
    assert!(data <= 0b11);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v = v & !(0b11 << (num * 2));
        v = v | (data << (num * 2));

        ptr::write_volatile(addr, v);
    }
}

fn priv_write_1bits(addr: *mut u32, num: u8, data: u32) {
    assert!(num < 32);
    assert!(data <= 0b1);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v = v & !(0b1 << num);
        v = v | (data << num);

        ptr::write_volatile(addr, v);
    }
}
fn priv_write_reg(addr: *mut u32, data: u32) {
    unsafe {
        ptr::write_volatile(addr, data);
    }
}
fn priv_read_1bits(addr: *mut u32, num: u8) -> bool {
    assert!(num < 32);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v &= 0b1 << num;

        return v != 0;
    }
}

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
}
