use core::ptr;

extern "C" {
    static _estack: u8;
    static _heap_start: u8;
    static _stack_reserve_start: u8;
    static _stack_reserve_end: u8;
}

pub fn usage() -> usize {
    let start: u32 = ptr::addr_of!(_heap_start) as u32;
    let end: u32 = ptr::addr_of!(_estack) as u32;

    let mut lower_bound = start;
    let mut upper_bound = end - 4;

    while upper_bound - lower_bound > 16 {
        let mid = (lower_bound + (upper_bound - lower_bound) / 2) & !0x3;
        let val: u32;
        unsafe {
            let addr = mid as *const u32;
            val = ptr::read_volatile(addr);
        }

        if val == 0xc1c1c1c1 {
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
