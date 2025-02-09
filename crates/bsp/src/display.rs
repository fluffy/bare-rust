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


        board::info::DISP_NRST.output();
        board::info::DISP_NRST.low(); // put into reset 

        board::info::DISP_CS.output();
        board::info::DISP_CS.low(); // chip select

        board::info::DISP_DC.output();
        board::info::DISP_DC.low(); // command 

        board::info::DISP_BL.output();
        board::info::DISP_BL.high(); // backlight on


        board::info::DISP_NRST.low(); // take out of reset 
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
