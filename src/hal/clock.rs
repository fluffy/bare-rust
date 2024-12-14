use core::ptr;

use super::cpu;
use super::cpu::*;
//use super::cpu::gen_cpu::*;

pub fn init() {
    // setup flash wait states and cache
    {
        // set latency to 5 wait states - NOTE, if voltage is changed, need to change this
        cpu::write!(FLASH.acr[LATENCY;3], 5);

        // enable data, instruction, prefetch cache
        cpu::write!(FLASH.acr[PRFTEN;1], 1);
        cpu::write!(FLASH.acr[ICEN;1], 1);
        cpu::write!(FLASH.acr[DCEN;1], 1);
    }

    // set up external clock and PLL
    {
        // enable HSE
        cpu::write!(RCC.cr[HSEON;1], 1);

        // wait for HSE to be ready
        while (cpu::read!(RCC.cr[HSERDY;1]) != 1) {}

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

        cpu::write!(RCC.pllcfgr[PLLQ0;4], pll_q );
        cpu::write!(RCC.pllcfgr[PLLM0;5], pll_m );
        cpu::write!(RCC.pllcfgr[PLLN0;9], pll_n );
        // set main division factor to 2
        cpu::write!(RCC.pllcfgr[PLLP0;2], 0b00);
        // select HSE
        cpu::write!(RCC.pllcfgr[PLLSRC;1], 0b1);

        // enable PLL
        cpu::write!(RCC.cr[PLLON;1], 0b1);
        // wait for PLL to be ready
        while (cpu::read!(RCC.cr[PLLRDY;1]) != 1) {}

        // setup clock usage and dividers
        // sys clock div 1
        cpu::write!(RCC.cfgr[HPRE;4], 0b0000);
        // APB1 Clk Div = 2
        cpu::write!(RCC.cfgr[PPRE1;3], 0b100);
        // APB2 Clk Div = 4
        cpu::write!(RCC.cfgr[PPRE2;3], 0b101);

        // switch clock to PLL
        cpu::write!(RCC.cfgr[SW0;2], 0b10 );

        // wait for clock to switch to PLL
        while (cpu::read!(RCC.cfgr[SWS0;2]) != 0b10) {}
    }

    // enable clocks for GPIO A,B,C
    {
        cpu::write!(RCC.ahb1enr[GPIOAEN;1], 1 );
        cpu::write!(RCC.ahb1enr[GPIOBEN;1], 1 );
        cpu::write!(RCC.ahb1enr[GPIOCEN;1], 1 );
    }
}
