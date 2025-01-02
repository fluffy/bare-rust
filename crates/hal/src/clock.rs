//! # Clock Module
//!
//! This module provides functions to initialize and validate the clock configuration for the system.
//! It handles setting up the PLL (Phase-Locked Loop), external clock, and various clock dividers.
//!
//! ## Functions
//!
//! - `init`: Initializes the clock configuration based on the board-specific settings.
//! - `validate`: Validates the clock configuration to ensure it is set up correctly.
//!
//! ## Usage
//!
//! The `init` function should be called during system startup to configure the clock. The `validate` function
//! can be used to check if the clock configuration is correct.
//!

use core::ptr;

//use super::board;
use super::cpu;
use super::cpu::*;

use super::board;

#[inline(never)]
/// Initializes the clock configuration based on the board-specific settings.
pub fn init() {
    if board::info::IS_SIM {
        cpu::write!(RCC.cfgr[SWS0;2], 0b10);
        cpu::write!(RCC.cr[PLLRDY;1], 1);
        cpu::write!(RCC.cr[HSERDY;1], 1);
        cpu::write!(RCC.pllcfgr[PLLSRC;1], 1);
        cpu::write!(RCC.pllcfgr[PLLM0;6], board::info::CLOCK_PLL_M as u32);
    }

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
        if board::info::HAS_RCC {
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
        if board::info::HAS_RCC {
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
        if board::info::HAS_RCC {
            while (cpu::read!(RCC.cfgr[SWS0;2]) != 0b10) {}
        }
    }
}

#[inline(never)]
/// Validates the clock configuration to ensure it is set up correctly.
pub fn validate() {
    // Check if HSE is ready
    if board::info::HAS_RCC {
        if cpu::read!(RCC.cr[HSERDY;1]) != 1 {
            panic!("HSE not ready");
        }
    }

    // Check if PLL is ready
    if board::info::HAS_RCC {
        if cpu::read!(RCC.cr[PLLRDY;1]) != 1 {
            panic!("PLL not ready");
        }
    }

    // Check if PLL source is HSE
    // seems like this can not be read after it is enabled
    //if cpu::read!(RCC.pllcfgr[PLLSRC;1]) != 1 {
    //    panic!("PLL source not set to HSE");
    //}

    // Check if PLL M is set correctly
    // seems like this can not be read after it is enabled
    //if read!(RCC.pllcfgr[PLLM0;6]) != board::info::CLOCK_PLL_M {
    //    panic!("PLL M not set correctly");
    //}

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
}
