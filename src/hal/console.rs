//use core::ptr;

#[cfg(not(feature = "std"))]
use core::arch::asm;

//use super::{board, cpu, uart};
use super::uart;

#[cfg(feature = "std")]
extern crate std;

pub fn init() {}

pub fn write(s: &[u8]) {
    #[cfg(feature = "std")]
    if cfg!(feature = "board-sim") {
        for c in s {
            std::print!("{}", *c as char);
        }
        return;
    }
    #[cfg(not(feature = "std"))]
    if cfg!(feature = "board-qemu") {
        // make data be null term version of s
        let mut data = [0u8; 80 + 1];
        for (i, c) in s.iter().enumerate() {
            if i > 80 { break; }
            data[i] = *c;
        }
        if s.len() > 80 {
            data[80] = b'\0';
        } else {
            data[s.len()] = 0;
        }
        
        let ptr = data.as_ptr();
      
        unsafe {
            // semihost WRITE0
            asm!(
            "mov r0, #0x04", // from https://github.com/ARM-software/abi-aa/blob/main/semihosting/semihosting.rst#sys-write0-0x04
            "mov r1, {0}",
            "bkpt #0xAB", // from https://github.com/ARM-software/abi-aa/blob/main/semihosting/semihosting.rst#4the-semihosting-interface
            in(reg) ptr,
            );
        }

        return;
    }
    for c in s {
        uart::write1(*c);
    }
}

pub fn write_u64(v: u64) {
    let mut num = v;
    let mut buffer = [0u8; 20 ]; // u64 max is 20 digits
    let mut len: usize = 0;

    if num == 0 {
        buffer[len] = b'0';
        len += 1;
    } else {
        while num > 0 {
            buffer[len] = (num % 10) as u8 + b'0';
            num /= 10;
            len += 1;
        }
    }

    let slice = &mut buffer[0..len];
    slice.reverse();
    
    write( slice );
}
