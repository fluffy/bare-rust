#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
mod tests {
    use dev::*;
    use dev::console::Print;
    
    #[test]
    fn test_init() {
        let mut bsp = BSP::new();
        bsp.init();

        let (_state, _changed) = bsp.buttons.read_ptt();
        let (_state, _changed) = bsp.buttons.read_ai();

        let message: &[u8] = b"Hello, world!";
        message.print_console();
        
        let number: u64 = 42;
        number.print_console();

        dev::debug::set(0, true);
        dev::debug::set(0, false);
        
        led::set(led::Color::Green);
    }
}

