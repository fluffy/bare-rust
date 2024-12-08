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


//#[allow(concat_idents)]
macro_rules! reg_set_bit {
    ($x:ident.$y:ident[$z:ident;$w:expr],  $data:expr  ) => {
        //let offset = $x::$y::$z;

        let offset = concat_idents!($x, _, $y, _, $z);
        let mut mask = (1u32 << $w) - 1;
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

macro_rules! reg_get_bit {
    ( $x:ident.$y:ident[$z:ident;$w:expr] ) => {{
        // TODO why is {{ needed here
        //let offset = $x::$y::$z;

        let offset: i32 = concat_idents!($x, _, $y, _, $z);
        let mask = (1u32 << $w) - 1;
        let mut val;

        unsafe {
            let addr = ptr::addr_of_mut!((*$x).$y);
            val = ptr::read_volatile(addr);
        }
        val = val >> offset;
        val = val & mask;
        val
    }};
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
    // setup flash wait states and cache
    {
        // set latency to 5 wait states - NOTE, if voltage is changed, need to change this
        reg_set_bit!(FLASH.acr[LATENCY;3], 5);

        // enable data, instruction, prefetch cache
        reg_set_bit!(FLASH.acr[PRFTEN;1], 1);
        reg_set_bit!(FLASH.acr[ICEN;1], 1);
        reg_set_bit!(FLASH.acr[DCEN;1], 1);
    }

    // set up external clock and PLL
    {
        // enable HSE
        reg_set_bit!(RCC.cr[HSEON;1], 1);

        // wait for HSE to be ready
        while (reg_get_bit!(RCC.cr[HSERDY;1]) != 1) {}

        // setup main PLL timing for external HSE
        #[cfg(feature = "brd-hactar-10")]
        let pll_m: u32 = 12;
        #[cfg(feature = "brd-blink-clk-a")]
        let pll_m: u32 = 8;
        let pll_n: u32 = 168;
        let pll_q: u32 = 4;

        assert!(pll_q >= 2);
        assert!(pll_q <= 0xF);
        assert!(pll_n >= 50);
        assert!(pll_n <= 432);
        assert!(pll_m >= 2);
        assert!(pll_m <= 63);

        reg_set_bit!(RCC.pllcfgr[PLLQ0;4], pll_q );
        reg_set_bit!(RCC.pllcfgr[PLLM0;5], pll_m );
        reg_set_bit!(RCC.pllcfgr[PLLN0;9], pll_n );
        // set main division factor to 2
        reg_set_bit!(RCC.pllcfgr[PLLP0;2], 0b00);
        // select HSE
        reg_set_bit!(RCC.pllcfgr[PLLSRC;2], 0b01);

        // enable PLL
        reg_set_bit!(RCC.cr[PLLON;1], 0b1);
        // wait for PLL to be ready
        while (reg_get_bit!(RCC.cr[PLLRDY;1]) != 1) {}

        // setup clock usage and dividers
        reg_set_bit!(RCC.cfgr[HPRE;4], 0b0000);
        // APB1 Clk Div = 1
        reg_set_bit!(RCC.cfgr[PPRE1;3], 0b100);
        // APB2 Clk Div = 4
        reg_set_bit!(RCC.cfgr[PPRE2;3], 0b101);
        // switch clock to PLL
        reg_set_bit!(RCC.cfgr[SW0;2], 0b10 );

        // wait for clock to switch to PLL
        while (reg_get_bit!(RCC.cfgr[SWS0;2]) != 0b10) {}
    }

    {
        // enable clocks for GPIO A,B,C
        reg_set_bit!(RCC.ahb1enr[GPIOAEN;1], 1 );
        reg_set_bit!(RCC.ahb1enr[GPIOBEN;1], 1 );
        reg_set_bit!(RCC.ahb1enr[GPIOCEN;1], 1 );
    }
}
