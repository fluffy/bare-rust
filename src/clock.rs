use core::ptr;

use crate::gen_cpu::*;

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

macro_rules! write_bits {
    ($x:ident.$y:ident, $mask:expr, $data:expr  ) => {
        unsafe {
            let addr = ptr::addr_of_mut!((*$x).$y);
            let mut v: u32 = ptr::read_volatile(addr);
            v &= !$mask;
            v |= $data;
            ptr::write_volatile(addr, v);
        }
    };
}

//#[allow(concat_idents)]
macro_rules! reg_set_bit {
    ($x:ident.$y:ident[$z:ident;$w:expr],  $data:expr  ) => {
        //let offset = $x::$y::$z;

        let offset = concat_idents!( $x, _, $y, _, $z );
        let mut mask = (1u32 << $w) -1;
        let mut val = $data & mask;
        mask = mask << offset;
        val = val << offset;
        unsafe {
            let addr = ptr::addr_of_mut!((*$x).$y);
            let mut v: u32 = ptr::read_volatile(addr);
            v &= !mask;
            v |= val;
            ptr::write_volatile(addr, v);
        }
    };
}

macro_rules! read_bits {
    ($x:ident.$y:ident, $mask:expr  ) => {
        unsafe {
            let addr = ptr::addr_of_mut!((*$x).$y);
            let mut v: u32 = ptr::read_volatile(addr);
            v &= !$mask;
            v
        }
    };
}

#[allow(non_snake_case)]
mod FLASH {
    pub mod acr {
        #![allow(unused)]
        
        pub const PRFTEN: u32 = 8;
        pub const ICEN: u32 = 9;
        pub const DCEN: u32 = 10;
    }
}



pub fn init() {
    //#[cfg(feature = "none")]
    {
        // setup flash wait states and cache
        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        // set latency to 5 wait states - NOTE, if voltage is changed, need to change this
        mask |= 0b111 << FLASH_acr_LATENCY;
        val |= 0b101 << FLASH_acr_LATENCY;
        write_bits!(FLASH.acr, mask, val);

        // enable data, instruction, prefetch cache
        reg_set_bit!(FLASH.acr[PRFTEN;1], 1);
        reg_set_bit!(FLASH.acr[ICEN;1], 1);
        reg_set_bit!(FLASH.acr[DCEN;1], 1);
    }

    //#[cfg(feature = "none")]
    {
        // enable HSE
        let mut val: u32 = 0;
        let mut mask: u32 = 0;
        mask |= 0b1 << RCC_cr_HSEON;
        val |= 0b1 << RCC_cr_HSEON;
        write_bits!(RCC.cr, mask, val);

        // wait for HSE to be ready
        loop {
            let mask: u32 = 0b1 << 17;
            let val: u32 = read_bits!(RCC.cr, mask);
            if val != 0 {
                break;
            }
        }

        // setup main PLL timing for external HSE
        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        #[cfg(feature = "brd-hactar-10")]
        let pll_m: u32 = 12;
        #[cfg(feature = "brd-blink-clk-a")]
        let pll_m: u32 = 8;
        let pll_n: u32 = 168;
        let pll_q: u32 = 4;

        mask |= 0b1 << 22;
        val |= 0b1 << 22; // select HSE
        mask |= 0b11 << 16;
        val |= 0b00 << 16; // set main division factor to 2

        assert!(pll_q >= 2);
        assert!(pll_q <= 0xF);
        assert!(pll_n >= 50);
        assert!(pll_n <= 432);
        assert!(pll_m >= 2);
        assert!(pll_m <= 63);

        mask |= 0b1111 << 24;
        val |= pll_q << 24;

        mask |= 0x7FFF << 0;
        val |= pll_n << 6;
        val |= pll_m;

        write_bits!(RCC.pllcfgr, mask, val);

        // wait for PLL to be ready
        loop {
            let mask: u32 = 0b1 << 25;
            let val = read_bits!(RCC.cr, mask);
            if val != 0 {
                break;
            }
        }

        // setup clock usage and dividers
        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        mask |= 0b11 << 0;
        val |= 0b10 << 0; // switch clock to PLL

        mask |= 0b1111 << 4;
        val |= 0b0000 << 4; // AHB Clk Div = 1

        mask |= 0b111 << 10;
        val |= 0b100 << 10; // APB1 CLk Div = 2

        mask |= 0b111 << 13;
        val |= 0b101 << 13; // APB2 Clk Div = 4

        write_bits!(RCC.cfgr, mask, val);

        // wait for clock to switch to PLL
        loop {
            let mask: u32 = 0b11 << 2;
            let val: u32 = read_bits!(RCC.cfgr, mask);
            if val == (0b10 << 2) {
                break;
            }
        }
    }

    {
        // enable clocks for GPIO A,B,C

        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        mask |= 0b111 << 0;
        val |= 0b111 << 0;

        write_bits!(RCC.ahb1enr, mask, val);
    }
}
