use core::ptr;

use super::cpu;
pub use super::cpu::*;

pub fn init() {}

pub fn is_enabled() -> bool {
    let v = cpu::read!( WWDG.cr[WDGA;1] );
    v != 0
}

#[cfg(feature = "stm32f072")]
pub fn start() {
    cpu::write!( DBGMCU.apb1_fz[DBG_WWDG_STOP;1],0b1);

    cpu::write!( WWDG.cfr[EWI;1]  , 0b0 );
    cpu::write!( WWDG.cfr[WDGTB;2]  , 0b11 );
    cpu::write!( WWDG.cfr[W;7]  , 0x7F );

    cpu::write!( WWDG.cr[T;7]  , 0x7F );
    cpu::write!( WWDG.cr[WDGA;1] , 0b1);
}

#[cfg(feature = "stm32f405")]
pub fn start() {
    cpu::write!( DBG.dbgmcu_apb1_fz[DBG_WWDG_STOP;1],0b1);

    cpu::write!( WWDG.cfr[EWI;1]  , 0b0 );
    cpu::write!( WWDG.cfr[WDGTB0;2]  , 0b11 );
    cpu::write!( WWDG.cfr[W;7]  , 0x7F );

    cpu::write!( WWDG.cr[T;7]  , 0x7F );
    cpu::write!( WWDG.cr[WDGA;1] , 0b1);
}

pub fn reset() {
    todo!();
}
