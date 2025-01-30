use core::ptr;

use super::cpu;
pub use super::cpu::*;

pub fn init() {}

pub fn is_enabled() -> bool {
    let v = cpu::read!( WWDG.cr[WDGA;1] );
    v != 0
}
