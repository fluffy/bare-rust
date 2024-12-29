
#[cfg(not(feature = "std"))]
use core::arch::asm;

use core::ptr;

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};


//use super::board;
use super::cpu;
use super::cpu::*;

pub use super::cpu::TIM_ADV as TIM1;

#[inline(never)]
pub fn init1() {
    // enable TIM1 clock
    cpu::write!( RCC.apb2enr[TIM1EN;1], 1);
   
    // apb2 timer clock is 168MHz
    
    // set prescaler for 1MHz
    cpu::write!(TIM1.psc, 168 - 1);

    // set auto-reload for 10ms
    cpu::write!(TIM1.arr, 10000 - 1);

    // force load of prescaler and auto-reload
    cpu::write!( TIM1.egr[UG;1], 1);

    // enable update interrupt
    cpu::write!( TIM1.dier[UIE;1], 1);

    // enable counter
    cpu::write!( TIM1.cr1[CEN;1], 1);

    // enable interrupt in NVIC
    const TIM1_UP_TIM10_IRQ_NUM: usize = 25;
    const IRQ_INDEX: usize = TIM1_UP_TIM10_IRQ_NUM / 32;
    const IRQ_BIT: usize = TIM1_UP_TIM10_IRQ_NUM % 32;

    cpu::write!(NVIC.iser[IRQ_INDEX], 1 << IRQ_BIT);
}

static mut TICK_COUNTER: u32 = 0;

pub fn handle_tim1_irq() {
    // clear update interrupt flag
    cpu::write!(TIM1.sr[UIF;1], 0);

    unsafe {
        // only the interrupt handler changes this and
        // interrupts disabled when it is read
        TICK_COUNTER += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MicroSeconds(pub u64);

impl MicroSeconds {
    pub fn new(microseconds: u64) -> Self {
        MicroSeconds(microseconds)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn sub(self, other: Self) -> MicroSeconds {
        if other.0 > self.0 {
            // wrap-around occurred
            return MicroSeconds( u64::MAX - other.0 + self.0 + 1);
        }
        MicroSeconds(self.0 - other.0)
    }
}

#[cfg( target_arch = "arm")]
#[inline(always)]
pub fn current_time() -> MicroSeconds {
    let lower: u32;
    let upper: u32;

    #[allow(unused_unsafe)] // the cpu::read! macro uses unsafe
    unsafe {
        // Some Arm processors do not support th cpsid and cpsie instructions
        // so we need to use the PRIMASK register to disable interrupts in that case - not done here
        
        // Read the value of the TIM1 counter
        asm!("cpsid i"); // disable interrupts
        lower = cpu::read!(TIM1.cnt);
        upper = TICK_COUNTER;
        asm!("cpsie i"); // enable interrupts
    }

    // Return the combined value
    MicroSeconds(((upper as u64) * 10_000) + (lower as u64))
}

#[cfg(feature = "std")]
pub fn current_time() -> MicroSeconds {
    
    let start = SystemTime::now();
    match start.duration_since(UNIX_EPOCH) {
        Ok(n) => MicroSeconds(n.as_micros() as u64),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}