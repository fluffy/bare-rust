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
    static mut _heap_start: u8;
}

#[no_mangle]
#[allow(static_mut_refs)]
pub extern "C" fn Reset_Handler() -> ! {
    unsafe {
        // initialize the BSS section with zeros
        let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
        ptr::write_bytes(&mut _sbss as *mut u8, 0, count);
    }
    unsafe {
        // initialize the DATA section with the data from the flash
        let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
        ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);
    }
    unsafe {
        // initialize the heap and stack to 0xC1 
        // leave 100 bytes free for this function
        let count = &_estack as *const u8 as usize - &_heap_start as *const u8 as usize - 100;
        ptr::write_bytes(&mut _heap_start as *mut u8, 0xC1, count);
    }
    
    unsafe { main() }
}

#[no_mangle]
pub extern "C" fn Default_Handler() -> ! {
    loop {}
}

#[cfg(target_arch = "arm")]
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static Reset_Vector: extern "C" fn() -> ! = Reset_Handler;

#[allow(dead_code)]
pub union IrqVector {
    not_used: u32,
    handler: unsafe extern "C" fn() -> !,
}

#[cfg(target_arch = "arm")]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static Exceptions: [IrqVector; 14] = [
    IrqVector {
        handler: Default_Handler,
    }, // NMI
    IrqVector {
        handler: Default_Handler,
    }, // hard fault
    IrqVector {
        handler: Default_Handler,
    }, // mem manager
    IrqVector {
        handler: Default_Handler,
    }, // bus fault
    IrqVector {
        handler: Default_Handler,
    }, // usage
    IrqVector { not_used: 0 },
    IrqVector { not_used: 0 },
    IrqVector { not_used: 0 },
    IrqVector { not_used: 0 },
    IrqVector {
        handler: Default_Handler,
    }, // SVC
    IrqVector { not_used: 0 }, // ebug mon
    IrqVector { not_used: 0 },
    IrqVector {
        handler: Default_Handler,
    }, // Pend  SV
    IrqVector {
        handler: Default_Handler,
    }, // Sys Timer
];
