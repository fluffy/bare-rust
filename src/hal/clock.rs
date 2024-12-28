use super::board;
use super::cpu;
use super::cpu::*;
use crate::board::info::HAS_RCC;
use core::ptr;
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
        if HAS_RCC {
            while (cpu::read!(RCC.cr[HSERDY;1]) != 1) {}
        }

        // setup main PLL timing for external HSE
        let pll_m: u32 = board::info::CLOCK_PLL_M;

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
        if HAS_RCC {
            while (cpu::read!(RCC.cr[PLLRDY;1]) != 1) {}
        }

        // setup clock usage and dividers
        // sys clock div 1
        cpu::write!(RCC.cfgr[HPRE;4], 0b0000);
        // APB1 Clk Div = 4
        cpu::write!(RCC.cfgr[PPRE1;3], 0b101);
        // APB2 Clk Div = 2
        cpu::write!(RCC.cfgr[PPRE2;3], 0b100);

        // switch clock to PLL
        cpu::write!(RCC.cfgr[SW0;2], 0b10 );

        // wait for clock to switch to PLL
        if HAS_RCC {
            while (cpu::read!(RCC.cfgr[SWS0;2]) != 0b10) {}
        }
    }

    // enable clocks for GPIO A,B,C
    {
        cpu::write!(RCC.ahb1enr[GPIOAEN;1], 1 );
        cpu::write!(RCC.ahb1enr[GPIOBEN;1], 1 );
        cpu::write!(RCC.ahb1enr[GPIOCEN;1], 1 );
    }
}

pub fn validate() {
    // Check if HSE is ready
    if cpu::read!(RCC.cr[HSERDY;1]) != 1 {
        panic!("HSE not ready");
    }

    // Check if PLL is ready
    if cpu::read!(RCC.cr[PLLRDY;1]) != 1 {
        panic!("PLL not ready");
    }

    // Check if PLL source is HSE
    if cpu::read!(RCC.pllcfgr[PLLSRC;1]) != 1 {
        panic!("PLL source not set to HSE");
    }

    // Check if PLL M is set correctly
    if read!(RCC.pllcfgr[PLLM0;6]) != board::info::CLOCK_PLL_M {
        panic!("PLL M not set correctly");
    }

    // Check if PLL N is set to 168
    if read!(RCC.pllcfgr[PLLN0;9]) != 168 {
        panic!("PLL N not set to 168");
    }

    // Check if PLL P is set to 2
    if read!(RCC.pllcfgr[PLLP0;2]) != 0 {
        panic!("PLL P not set to 2");
    }

    // Check if PLL Q is set to 4
    if read!(RCC.pllcfgr[PLLQ0;4]) != 4 {
        panic!("PLL Q not set to 4");
    }

    // Check if system clock mux is set to PLL
    if cpu::read!(RCC.cfgr[SWS0;2]) != 0b10 {
        panic!("System clock not set to PLL");
    }

    // Check AHB prescaler
    if cpu::read!(RCC.cfgr[HPRE;4]) != 0b0000 {
        panic!("AHB prescaler not set to 1");
    }

    // Check APB1 prescaler
    if cpu::read!(RCC.cfgr[PPRE1;3]) != 0b101 {
        panic!("APB1 prescaler not set to 4");
    }

    // Check APB2 prescaler
    if cpu::read!(RCC.cfgr[PPRE2;3]) != 0b100 {
        panic!("APB2 prescaler not set to 2");
    }

    // Output system clock on MCO1 pin (PA8)
    if false {
        // enable GPIO A
        cpu::write!(RCC.ahb1enr[GPIOAEN;1], 1 );
        
        // set PA8 to alternate function
        cpu::write!(GPIO_A.moder[8*2;2], 0b10);
        // set PA8 to AF0
        cpu::write!(GPIO_A.afrh[0;4], 0b0000);
        
        // set MCO1 to PLL clock
        cpu::write!(RCC.cfgr[MCO1;2], 0b11); 
        
        // set MCO1 prescaler to 1
        cpu::write!(RCC.cfgr[MCO1PRE;3], 0b000); 
        
        loop {
           // wait forever 
        }
    }
}
