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
