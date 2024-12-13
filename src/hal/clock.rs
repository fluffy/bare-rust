use core::ptr;

//use crate::cpu::gen_cpu::*;
use super::cpu::*;




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
        reg_set_bit!(RCC.pllcfgr[PLLSRC;1], 0b1);

        // enable PLL
        reg_set_bit!(RCC.cr[PLLON;1], 0b1);
        // wait for PLL to be ready
        while (reg_get_bit!(RCC.cr[PLLRDY;1]) != 1) {}

        // setup clock usage and dividers
        // sys clock div 1
        reg_set_bit!(RCC.cfgr[HPRE;4], 0b0000);
        // APB1 Clk Div = 2
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
