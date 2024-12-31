#![no_std]

extern crate hal;

pub mod buttons;
pub mod console;
pub mod debug;
pub mod led;

pub  struct BSP{
    pub  button: buttons::Buttons,
    pub  console: console::Console,
    pub  debug: debug::Debug,
    pub  led: led::Led,
}

impl BSP {
    pub fn new() -> Self {
        BSP {
            button: buttons::Buttons::new(),
            console: console::Console::new(),
            debug: debug::Debug::new(),
            led: led::Led::new(),
        }
    }

    pub fn init(&mut self) {
        hal::init();

        // do early so other modules can use it
        self.console.init();
        self.led.init();
        self.debug.init();
        self.button.init();
    }

    pub fn validate(&self) {
        hal::validate();
        
        self.button.validate();
    }
}
