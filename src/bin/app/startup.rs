
use core::ptr;


extern "C" {
    fn main() -> !;

}

extern "C" {
    static mut _sbss: u8;
    static _ebss: u8;
    static mut _sdata: u8;
    static _edata: u8;
    static _sidata: u8;
    static _estack: u8;
    static _heap_start: u8;
}


#[no_mangle]
#[allow(static_mut_refs)]
pub extern "C" fn Reset_Handler() -> ! {
    unsafe {
        let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
        ptr::write_bytes(&mut _sbss as *mut u8, 0, count);
    }
    unsafe {
        let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
        ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);
    }

    unsafe { main() }
}