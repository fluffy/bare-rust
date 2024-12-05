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

pub const GPIO_A: *mut GpioReg = (0x4800_0800 + 0x400 * 0) as *mut GpioReg;
pub const GPIO_B: *mut GpioReg = (0x4800_0800 + 0x400 * 1) as *mut GpioReg;
pub const GPIO_C: *mut GpioReg = (0x4800_0800 + 0x400 * 2) as *mut GpioReg;

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
    assert!(num < 16);
    assert!(data <= 0b1);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v = v & !(0b1 << num);
        v = v | (data << num);

        ptr::write_volatile(addr, v);
    }
}
fn priv_read_1bits(addr: *mut u32, num: u8) -> bool {
    assert!(num < 16);
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

        // set output as low
        write_1bits!(gpio.odr, pin_num, 0b0);
        // set as push pull
        write_1bits!(gpio.otyper, pin_num, 0b0);
        // set no pull up , no pull down
        write_2bits!(gpio.pupdr, pin_num, 0b00);
        // set speed to slow
        write_2bits!(gpio.ospeedr, pin_num, 0b00);
        // set mode to output
        write_2bits!(gpio.moder, pin_num, 0b10);
    }

    fn input(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        // set to pull down
        write_2bits!(gpio.pupdr, pin_num, 0b10);
        // set mode to input
        write_2bits!(gpio.moder, pin_num, 0b00);
    }
    pub fn high(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        write_1bits!(gpio.bsrr, pin_num + 16, 0b1);
    }
    pub fn low(&self) {
        let gpio = self.0;
        let pin_num = self.1;

        write_1bits!(gpio.bsrr, pin_num, 0b1);
    }

    pub fn read(&self) -> bool {
        let gpio = self.0;
        let pin_num = self.1;

        return read_1bits!(gpio.idr, pin_num);
    }
}

pub fn init() {
    Pin::new(GPIO_A, 6).output(); // red LED
    Pin::new(GPIO_C, 5).output(); // green LED
    Pin::new(GPIO_A, 1).output(); // blue LED

    Pin::new(GPIO_C, 0).input(); // PTT
}
