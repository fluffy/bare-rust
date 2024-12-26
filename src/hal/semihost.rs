
#[cfg(feature = "std")]
extern crate std;

#[cfg(target_arch = "arm")]
use core::arch::asm;

#[cfg(target_arch = "arm")]
pub fn exit_no_status() -> ! {
    unsafe {
        asm!(
        "mov r0, #0x18",
        //"mov r1, #0x20026",
        "movw r1, #0x0026", // Move lower half
        "movt r1, #0x2",    // Move upper half
        "bkpt #0xAB"
        );
    }
    loop {}
}

#[cfg(target_arch = "arm")]
pub fn exit( ret : i32) -> ! {

    #[repr(C, packed)]
    struct Args {
        reason: u32,
        status: i32,
    }
    const ADP_STOPPED_APPLICATION_EXIT:u32 = 0x20026; // from https://github.com/ARM-software/abi-aa/blob/main/semihosting/semihosting.rst#sys-exit-0x18
    let args =  Args {
        reason:ADP_STOPPED_APPLICATION_EXIT,
        status:ret,
    };
    let arg_ptr: *const Args = &args;

    unsafe {
        asm!(
        "mov r0, #0x20", // from https://github.com/ARM-software/abi-aa/blob/main/semihosting/semihosting.rst#66sys_exit_extended-0x20
        "mov r1, {0}",
        "bkpt #0xAB", // from https://github.com/ARM-software/abi-aa/blob/main/semihosting/semihosting.rst#4the-semihosting-interface
        in(reg) arg_ptr,
        );
    }
    loop {}
}

#[cfg(not(target_arch = "arm"))]
pub fn exit( ret : i32) -> ! {
    std::process::exit( ret );
}