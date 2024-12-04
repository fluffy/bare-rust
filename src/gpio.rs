use core::ptr;

#[derive(Copy, Clone)]
pub enum Bank {
    A = 0,
    B = 1,
    C = 2,
}

#[derive(Copy, Clone)]
pub struct Pin(Bank, u8);

#[repr(C)]
struct GpioReg {
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

fn gpio(b: Bank) -> *mut GpioReg {
    let bank_num = b as u8;
    let addr: u32 = 0x4001_0800 + 0x400 * bank_num as u32;
    let gpio_ptr: *mut GpioReg = addr as *mut GpioReg;
    return gpio_ptr;
}

fn init_output(p: Pin) {
    let gpio = gpio(p.0);
    let pin_num = p.1;

    unsafe {
        // set mode to output 
        ptr::write_volatile(((*gpio).moder) as *mut u32, 0x01 << (pin_num * 2));
    }
}

pub fn init() {
    init_output(Pin(Bank::A, 1));
}

impl Pin {
    pub fn new(b: Bank, p: u8) -> Pin {
        assert!(p < 16);
        return Pin(b, p);
    }

    pub fn high(&self) {
        todo!();
    }
    pub fn low(&self) {
        todo!();
    }
}
