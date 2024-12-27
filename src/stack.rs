#[cfg(target_arch = "arm")]
use core::ptr;

extern "C" {
    static _estack: u8;
    static _heap_start: u8;
    static _stack_reserve_start: u8;
    static _stack_reserve_end: u8;
}

#[cfg(target_arch = "arm")]
pub fn usage() -> usize {
    let start: u32 = ptr::addr_of!(_heap_start) as u32;
    let end: u32 = ptr::addr_of!(_estack) as u32;

    let mut lower_bound = start;
    let mut upper_bound = end - 8;

    while upper_bound - lower_bound > 16 {
        let mid = (lower_bound + (upper_bound - lower_bound) / 2) & !0x3;
        let val1: u32;
        let val2: u32;
        unsafe {
            let addr1 = mid as *const u32;
            val1 = ptr::read_volatile(addr1);
            let addr2 = (mid + 4) as *const u32;
            val2 = ptr::read_volatile(addr2);
        }

        if (val1 == 0xc5c5c5c5) && (val2 == 0xc5c5c5c5) {
            lower_bound = mid;
        } else {
            upper_bound = mid;
        }
    }

    let reserve_start: u32 = ptr::addr_of!(_stack_reserve_start) as u32;
    let reserve_end: u32 = ptr::addr_of!(_stack_reserve_end) as u32;
    let reserved: usize = (reserve_end - reserve_start) as usize;

    let usage = (end - lower_bound) as usize;

    if usage > reserved {
        panic!("Stack overflow");
    }

    usage
}

#[cfg(not(target_arch = "arm"))]
pub fn usage() -> usize {
    0
}
