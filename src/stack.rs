use core::ptr;

extern "C" {
    static _estack: u8;
    static _heap_start: u8;
}

pub fn stack_usage() -> usize {
    let start: u32 = ptr::addr_of!(_heap_start) as u32;
    let end: u32 = ptr::addr_of!(_estack) as u32;
   
    let mut lower_bound = start;
    let mut upper_bound = end-4;
    
    while upper_bound - lower_bound > 4 {
        let mid = lower_bound + (upper_bound - lower_bound) / 2;
        unsafe 
            {
            let addr = mid as *mut u32;
         
            if ptr::read_volatile(addr) == 0xc1c1c1c1 {
                lower_bound = mid;
            } else {
                upper_bound = mid;
            }
        }
    }
    
    return (end - lower_bound) as usize;
}
