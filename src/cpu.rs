mod gen_cpu;

use core::ptr;

pub use crate::cpu::gen_cpu::*;

#[repr(C)]
pub struct FlashReg {
    pub acr: u32,
    pub keyr: u32,
    pub opkeyr: u32,
    pub sr: u32,
    pub cr: u32,
    pub optcr: u32,
}

pub const FLASH: *mut FlashReg = 0x4002_3C00 as *mut FlashReg;

#[repr(C)]
pub struct RccReg {
    pub cr: u32,
    pub pllcfgr: u32,
    pub cfgr: u32,
    pub cir: u32,

    pub ahb1rstr: u32,
    pub ahb2rstr: u32,
    pub ahb3rstr: u32,
    reserved1: u32,

    pub apb1_enr: u32,
    pub apb12enr: u32,
    reserved2: u32,
    reserved3: u32,

    pub ahb1enr: u32,
    pub ahb2enr: u32,
    pub ahb3enr: u32,
    reserved4: u32,

    pub apb1enr: u32,
    pub apb2enr: u32,
    reserved5: u32,
    reserved6: u32,

    pub ahb1lpenr: u32,
    pub ahb2lpenr: u32,
    pub ahb3lpenr: u32,
    reserved7: u32,

    pub apb1lpenr: u32,
    pub apb2lpenr: u32,
    reserved8: u32,
    reserved9: u32,

    pub bdcr: u32,
    pub csr: u32,
    reserved10: u32,
    reserved11: u32,

    pub sscgr: u32,
    pub pli2scfgr: u32,
}

pub const RCC: *mut RccReg = 0x4002_3800 as *mut RccReg;



#[allow(non_snake_case)]
pub mod FLASH {
    pub mod acr {
        #![allow(unused)]

        pub const PRFTEN: u32 = 8;
        pub const ICEN: u32 = 9;
        pub const DCEN: u32 = 10;
    }
}

#[repr(C)]
pub struct GpioReg {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
    pub brr: u32,
}

pub const GPIO_A: *mut GpioReg = 0x4002_0000 as *mut GpioReg;
pub const GPIO_B: *mut GpioReg = 0x4002_0400 as *mut GpioReg;
pub const GPIO_C: *mut GpioReg = 0x4002_0800 as *mut GpioReg;

pub fn priv_write_2bits(addr: *mut u32, num: u8, data: u32) {
    assert!(num < 16);
    assert!(data <= 0b11);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v = v & !(0b11 << (num * 2));
        v = v | (data << (num * 2));

        ptr::write_volatile(addr, v);
    }
}

pub fn priv_write_1bits(addr: *mut u32, num: u8, data: u32) {
    assert!(num < 32);
    assert!(data <= 0b1);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v = v & !(0b1 << num);
        v = v | (data << num);

        ptr::write_volatile(addr, v);
    }
}
pub fn priv_write_reg(addr: *mut u32, data: u32) {
    unsafe {
        ptr::write_volatile(addr, data);
    }
}
pub fn priv_read_1bits(addr: *mut u32, num: u8) -> bool {
    assert!(num < 32);
    unsafe {
        let mut v: u32 = ptr::read_volatile(addr);

        v &= 0b1 << num;

        return v != 0;
    }
}
