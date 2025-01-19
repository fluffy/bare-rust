//! Application main loop and entry point

#![no_std]
#![no_main]

//extern crate bsp;
extern crate hal;

use hal::{cpu, gpio};
//use bsp::console::Print;
//use bsp::led;
//use bsp::led::Color;

mod stack;
mod startup;

#[no_mangle]
#[export_name = "main"]
#[inline(never)]
/// Entry point for the application when the `std` feature is not enabled.
pub extern "C" fn main() -> ! {
    my_main();

    loop {}
}

#[inline(never)]
/// Main function that initializes the system and runs the task manager.
fn my_main() {
    pub const CONSOLE_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 9);
    pub const CONSOLE_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 10);

    pub const UI_TX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 2);
    pub const UI_RX: gpio::Pin = gpio::Pin(cpu::GPIO_A, 3);

    pub const UI_BOOT0: gpio::Pin = gpio::Pin(cpu::GPIO_A, 15);
    pub const UI_NRST: gpio::Pin = gpio::Pin(cpu::GPIO_B, 3);

    pub const NET_BOOT0: gpio::Pin = gpio::Pin(cpu::GPIO_B, 5);
    pub const NET_NRST: gpio::Pin = gpio::Pin(cpu::GPIO_B, 4);

    pub const MCLK: gpio::Pin = gpio::Pin(cpu::GPIO_A, 8);
    pub const MCLK_FREQ: u32 = 24_000_000;
    pub const CLOCK_HSE_FREQ: u32 = 16_000_000; // set to 0 for simulation

    hal::init(CLOCK_HSE_FREQ, CONSOLE_TX, CONSOLE_RX);

    hal::uart::init2(115200, UI_RX, UI_TX);

    const LED_RED_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 4);
    const LED_GREEN_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 6);
    const LED_BLUE_PIN: gpio::Pin = gpio::Pin(cpu::GPIO_A, 7);

    LED_GREEN_PIN.output();
    LED_RED_PIN.output();
    LED_BLUE_PIN.output();

    LED_GREEN_PIN.high();
    LED_RED_PIN.high();
    LED_BLUE_PIN.low(); // turn on blue LED

    hal::clock::configure_mco(MCLK, MCLK_FREQ);

    // make sur boot are low before any rest
    UI_BOOT0.output();
    NET_BOOT0.output();
    UI_BOOT0.low();
    NET_BOOT0.low();

    UI_NRST.open_drain();
    UI_NRST.pullup();
    NET_NRST.open_drain();
    NET_NRST.pullup();

    UI_NRST.low();
    NET_NRST.low();

    let str = "MGMT: Starting\r\n";
    for c in str.bytes() {
        hal::uart::write1(c);
    }

    UI_NRST.high();
    NET_NRST.high();

    LED_GREEN_PIN.low(); // turn on green LED
    LED_RED_PIN.high();
    LED_BLUE_PIN.high();

    let (stack_usage, stack_current, stack_reserved) = stack::usage(false);
    let _ = (stack_usage, stack_current, stack_reserved);

    loop {
        let c: u8;
        c = hal::uart::read2();
        hal::uart::write1(c);
    }
}
