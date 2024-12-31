use core::ptr;

#[cfg(not(feature = "std"))]
use core::arch::asm;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

use super::cpu;
use super::cpu::*;

pub use super::cpu::TIM_GEN as TIM2;

#[inline(never)]
pub fn init1() {
    // enable TIM2 clock
    //cpu::write!( RCC.apb2enr[TIM2EN;1], 1);
    cpu::write!( RCC.apb1enr[TIM2EN;1], 1);

    // apb2 timer clock is 168MHz

    // set prescaler for 1MHz
    cpu::write!(TIM2.psc, 168 - 1);

    // set auto-reload for 10ms
    cpu::write!(TIM2.arr, 10000 - 1);

    // force load of prescaler and auto-reload
    cpu::write!( TIM2.egr[UG;1], 1);

    // enable update interrupt
    cpu::write!( TIM2.dier[UIE;1], 1);

    // enable counter
    cpu::write!( TIM2.cr1[CEN;1], 1);

    // enable interrupt in NVIC
    const TIM2_IRQ_NUM: usize = 28;
    const IRQ_INDEX: usize = TIM2_IRQ_NUM / 32;
    const IRQ_BIT: usize = TIM2_IRQ_NUM % 32;

    cpu::write!(NVIC.iser[IRQ_INDEX], 1 << IRQ_BIT);
}

static mut TICK_COUNTER: u32 = 0;

#[inline(never)]
pub fn handle_tim2_irq() {
    // clear update interrupt flag
    cpu::write!(TIM2.sr[UIF;1], 0);

    unsafe {
        // only the interrupt handler changes this and
        // interrupts disabled when it is read
        let mut v: u32 = core::ptr::read_volatile( &TICK_COUNTER );
        v+= 1;
        core::ptr::write_volatile( &mut TICK_COUNTER, v);
        //TICK_COUNTER += 1;
    }
}

#[cfg(target_arch = "arm")]
#[inline(never)]
pub fn current_time() -> MicroSeconds {
    let lower: u32;
    let upper: u32;

    #[allow(unused_unsafe)] // the cpu::read! macro uses unsafe
    unsafe {
        // Some Arm processors do not support th cpsid and cpsie instructions
        // so we need to use the PRIMASK register to disable interrupts in that case - not done here

        // Read the value of the TIM2 counter
        asm!("cpsid i"); // disable interrupts
        let mut v: u32 = core::ptr::read_volatile( &TICK_COUNTER );
        upper = v;
        lower = cpu::read!(TIM2.cnt);
        //upper = TICK_COUNTER;
        asm!("cpsie i"); // enable interrupts
    }

    // Return the combined value
    MicroSeconds(((upper as u64) * 10_000) + (lower as u64))
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
            // wrap-around occurred ( happens about every 12 hours )
            let max: u64 = (u32::MAX as u64) * 10_000;
            return MicroSeconds(max - other.0 + self.0 + 1);
        }
        MicroSeconds(self.0 - other.0)
    }
}

#[cfg(feature = "std")]
pub fn current_time() -> MicroSeconds {
    let start = SystemTime::now();
    match start.duration_since(UNIX_EPOCH) {
        Ok(n) => MicroSeconds((n.as_micros() as u64) % ((u32::MAX as u64) * 10_000)),
        Err(_) => panic!("bad system time"),
    }
}
