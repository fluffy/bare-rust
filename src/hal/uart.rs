use core::ptr;
//use crate::board::info::HAS_RCC;
use super::board;
//use super::gpio;
use super::cpu;
use super::cpu::*;

pub use super::cpu::USART as USART1;

pub fn init() {
   
    // enable USART1 & GPIO clock 
    cpu::write!( RCC.apb2enr[USART1EN;1], 1);
    cpu::write!( RCC.ahb1enr[GPIOAEN;1], 1);
    
    // configure pins PA9,PA19 for USART1
    // AF7 work for USART1 to 3. afrh work pin 8 to 15
    cpu::write!( GPIO_A.moder[9*2;2], 0b10); // AF mode 
    cpu::write!( GPIO_A.moder[10*2;2], 0b10); // AF mode 
    cpu::write!( GPIO_A.afrh[(9-8)*4;4], 7); // AF7 mode 
    cpu::write!( GPIO_A.afrh[(10-8)*4;4], 7); // AF7 mode 
    
    // set baud rate to 115200
    // UART 1 is on APB2 bus, which is 84MHz
    let apb_freq :u64 = 84_000_000;
    let baud_rate :u64 = 115_200 ; 
    let div_fixed3 :u64 = 1000*apb_freq / (16* baud_rate );
  
    let mantissa :u64 = div_fixed3 / 1000;
    let frac: u64 = (div_fixed3%1000)*16/1000;

    cpu::write!( USART1.brr[DIV_Mantissa;11], mantissa as u32);
    cpu::write!( USART1.brr[DIV_Fraction;4], frac as u32);

    cpu::write!( USART1.cr1[M;1], 0); // 8 data bits
    cpu::write!( USART1.cr1[PCE;1], 0); // no parity
    cpu::write!( USART1.cr2[STOP;2], 0b00); // 1 stop bit

    cpu::write!( USART1.cr1[TE;1], 1); // transmit enable
    cpu::write!( USART1.cr1[RE;1], 1); // receive enable
    cpu::write!( USART1.cr1[UE;1], 1); // uart enable 
  
}

pub fn write(c: u8) {
    while (cpu::read!(USART1.sr[TXE;1]) == 0) {}
    cpu::write!(USART1.dr[DR;8], c as u32);
}

pub fn write_slice( s: &[u8] ) {
    for c in s {
        write(*c);
    }
}