//! # Display Module
//!
//! This module provides functionality for controlling the display on the board.
//!
//! ## Functions
//!
//! - `size`: Returns the width and height of the display.
//! - `draw_bitmap`: Draws a bitmap at a specified position.
//!
//! ## Example
//!
//! ```rust
//! use bsp::display::Display;
//!
//! let display = Display::new();
//! display.init();
//!
//! display.draw_bitmap( &[0xFFFF; 240 * 320/10], 0,0, 240, 320/10, 240 );
//!
//! while !display.ready() {}
//! ```
//!

extern crate hal;

#[cfg(feature = "std")]
extern crate std;

use crate::board;

const WIDTH: usize = 240;
const HEIGHT: usize = 320;

/// Struct keeping track of the display state
pub struct Display {}

impl Display {
    #[inline(never)]
    pub fn new() -> Self {
        Display {}
    }

    #[inline(never)]
    pub fn init(&self) {
        board::info::DISP_CS.output();
        board::info::DISP_CS.low(); // chip select

        board::info::DISP_DC.output();
        board::info::DISP_DC.high(); // high for data

        board::info::DISP_BL.output();
        board::info::DISP_BL.high(); // backlight on

        board::info::DISP_NRST.output();
        board::info::DISP_NRST.low(); // put into reset
        let now = hal::timer::current_time();
        while hal::timer::current_time().sub( now ).as_u64() < 50_000 {} // TODO needed ?
        board::info::DISP_NRST.high(); // take out of reset
        let now = hal::timer::current_time();
        while hal::timer::current_time().sub( now ).as_u64() < 120_000 {} // TODO needed ?

        hal::spi::init1(
            board::info::DISP_SPI_FREQ,
            board::info::DISP_SCL,
            board::info::DISP_SDA,
        );

        ili9341::setup();
    }

    /// Returns the width and height of the display.
    pub fn size(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }

    /// Returns whether the display is ready to accept the next draw_bitmap command.
    pub fn ready(&self) -> bool {
        true
    }

    /// Draws a bitmap at a specified position.
    ///
    /// This is an async call that starts a DMA transfer to the display controller and
    /// ready will return false until the transfer is complete.
    pub fn draw_bitmap(
        &self,
        bitmap: &[u16],
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        stride: usize,
    ) {
        // Draw a bitmap at position (x, y)
        let _unused = (bitmap, x, y, width, height, stride);
    }
}

mod ili9341 {
    use crate::board;


    #[allow(dead_code)]
    pub enum Command {
        NoOp = 0x00,       // No operation
        SwReset = 0x01,    // Software reset
        SleepOut = 0x11,   // Sleep out
        NormalMode = 0x13, // Normal mode

        GammaSet = 0x26,      // Gamma set
        DisplayOff = 0x28,    // Display off
        DisplayOn = 0x29,     // Display on
        ColumnAddrSet = 0x2A, // Column address set
        PageAddrSet = 0x2B,   // Page address set
        MemoryWrite = 0x2C,   // Memory write

        PixelFormatSet = 0x3A,   // Pixel format set
        MemoryAccessCtrl = 0x36, // Memory access control

        SetTearingEffectLineOff = 0x34, // Tearing effect line off
        SetTearingEffectLineOn = 0x35,  // Tearing effect line on

        SetTearScanline = 0x44, // Set tear scanline

        FrameRateCtrl = 0xB1, // Frame rate control (In normal mode/Full colors)
        DisplayFunctionCtrl = 0xB6, // Display function control

        PowerCtrl1 = 0xC0, // Power control 1
        PowerCtrl2 = 0xC1, // Power control 2 (C1h)
        VcomCtrl1 = 0xC5,  // VCOM control 1 (C5h)
        VcomCtrl2 = 0xC7,  // VCOM control 2 (C7h)
        PowerCtrlA = 0xCB, // Power control A (CBh)
        PowerCtrlB = 0xCF, // Power control B (CFh)

        PositiveGammaCorrect = 0xE0, // Positive gamma correction
        NegativeGammaCorrect = 0xE1, // Negative gamma correction

        DriverTimingCtrlA = 0xE8, // Driver timing control A (E8h)
        DriverTimingCtrlB = 0xEA, // Driver timing control B (EAh)
        PowerOnSeqCtrl = 0xED,    // Power on sequence control (EDh)

        Enable3GammaCtrl = 0xF2, // Enable 3 gamma control (F2h)
        PumpRatioCtrl = 0xF7,    // Pump ratio control (F7h)
    }

    pub fn command(cmd: Command, paramters: &[u8]) {
        // Send a command to the display
        let c: [u8; 1] = [cmd as u8];

        board::info::DISP_DC.low(); // command
        hal::spi::write1(&c);

        board::info::DISP_DC.high(); // data
        hal::spi::write1(paramters);
    }

    pub fn setup() {
        
        //LCD_2IN4_Write_Command(0x01); //Software reset
        command(Command::SwReset, &[]);
        let now = hal::timer::current_time();
        while hal::timer::current_time().sub( now ).as_u64() < 5_000 {}

        command(Command::PowerCtrlA, &[0x39, 0x2C, 0x00, 0x34, 0x02]);

        command(Command::PowerCtrlB, &[0x00, 0xC1, 0x30]);

        //command(Command::DriverTimingCtrlA, &[0x85, 0x00, 0x79]); // 78 or 79 ???
        command(Command::DriverTimingCtrlA, &[0x85, 0x00, 0x78]);

        command(Command::DriverTimingCtrlB, &[0x00, 0x00]);

        command(Command::PowerOnSeqCtrl, &[0x64, 0x03, 0x12, 0x81]);

        command(Command::PumpRatioCtrl, &[0x20]);

        //command(Command::PowerCtrl1, &[0x1D]); // or 0x23 ???
        command(Command::PowerCtrl1, &[0x23]);

        //command(Command::PowerCtrl2, &[0x12]); // or 0x10 ???
        command(Command::PowerCtrl2, &[0x10]);

        //command(Command::VcomCtrl1, &[0x33, 0x3F]); // or 3e,28
        command(Command::VcomCtrl1, &[0x3e, 0x28]);

        //command(Command::VcomCtrl2, &[0x92]); // or 86 ???
        command(Command::VcomCtrl2, &[0x86]);

        //command(Command::MemoryAccessCtrl, &[0x08]); // or 0x48 or 0x88
        command(Command::MemoryAccessCtrl, &[0x48]); // or 0x48 or 0x88

        command(Command::PixelFormatSet, &[0x55]);

        //command(Command::FrameRateCtrl, &[0x00, 0x12]); // or 18
        command(Command::FrameRateCtrl, &[0x00, 0x18]);

        // TODO takes 4 parameters
        //command(Command::DisplayFunctionCtrl, &[0x0A, 0xA2]); // perhaps  0x08, 0x82, 0x27 ???
        command(Command::DisplayFunctionCtrl, &[0x08, 0x82, 0x27]);

        // command(Command::SetTearScanline, &[0x02]);

        command(Command::Enable3GammaCtrl, &[0x00]);

        command(Command::GammaSet, &[0x01]);

        command(
            Command::PositiveGammaCorrect,
            &[
                // 0x0F, 0x22, 0x1C, 0x1B, 0x08, 0x0F, 0x48, 0xB8, 
                // 0x34, 0x05, 0x0C, 0x09, 0x0F, 0x07, 0x00
                // alternative values
                0x0F, 0x31, 0x2B, 0x0C, 0x0E, 0x08, 0x4E, 0xF1, 
                0x37, 0x07, 0x10, 0x03, 0x0E, 0x09, 0x00
            ],
        );

        command(
            Command::NegativeGammaCorrect,
            &[
                // 0x00, 0x23, 0x24, 0x07, 0x10, 0x07, 0x38, 0x47,
                // 0x4B, 0x0A, 0x13, 0x06, 0x30, 0x38, 0x0F
                // alternative values
                0x00, 0x0E, 0x14, 0x03, 0x11, 0x07, 0x31, 0xC1, 
                0x48, 0x08, 0x0F, 0x0C, 0x31, 0x36, 0x0F
            ],
        );

        command(Command::NormalMode, &[]);

        command(Command::SleepOut, &[]);
        let now = hal::timer::current_time();
        while hal::timer::current_time().sub( now ).as_u64() < 120_000 {}

        command(Command::DisplayOn, &[]);

        //loop { 
        if true {
            command(Command::ColumnAddrSet, &[0x00, 10, 0x00, 100]); // EF=239
            command(Command::PageAddrSet, &[0x00, 10, 0x00, 60]); // 13F=319

            let data = [0xFFu8; 90 * 40 * 2];
            command(Command::MemoryWrite, &data);
            command(Command::NoOp, &[]);
        }
    }
}
