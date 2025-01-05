//! # Display Module
//!
//! This module provides functionality for controlling the display on the board.
//! It includes methods for initializing the display, drawing pixels, bitmaps, and rectangles
//! It also has function that needs to be periodically called for refreshing the display to the LCD.
//!
//! ## Functions
//!
//! - `size`: Returns the width and height of the display.
//! - `draw_pixel`: Draws a pixel at a specified position.
//! - `draw_bitmap`: Draws a bitmap at a specified position.
//! - `draw_rect`: Draws a rectangle at a specified position with the specified width, height, and color.
//! - `refresh`: Refreshes the display.
//!
//! ## Example
//!
//! ```rust
//! use bsp::display::Display;
//!
//! let display = Display::new();
//! display.init();
//!
//! display.draw_pixel(10, 10, 0xFFFF);
//! display.draw_bitmap(0, 0, &[0xFFFF; 128 * 64], 128, 64, 128);
//! display.draw_rect(20, 20, 50, 50, 0xFFFF);
//!
//! display.refresh();
//! ```

extern crate hal;

#[cfg(feature = "std")]
extern crate std;

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

/// Struct keeping track of the display state
pub struct Display {
    /// bitmap buffer for the display
    vram: [u16; WIDTH * HEIGHT],
    /// dirty flag indicating that row needs to be refreshed to LCD
    dirty: [bool; HEIGHT],
}

impl crate::display::Display {
    #[inline(never)]
    pub fn new() -> Self {
        crate::display::Display {
            vram: [0; WIDTH * HEIGHT],
            dirty: [true; HEIGHT],
        }
    }

    #[inline(never)]
    pub fn init(&self) {}

    /// Returns the width and height of the display.
    pub fn size(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }

    /// Draws a pixel at a specified position.
    pub fn draw_pixel(&self, x: u32, y: u32, color: u16) {
        // Draw a pixel at position (x, y)
        let _unused = (x, y, color);
    }

    /// Draws a bitmap at a specified position.
    pub fn draw_bitmap(
        &self,
        x: u32,
        y: u32,
        bitmap: &[u16],
        width: u32,
        height: u32,
        stride: u32,
    ) {
        // Draw a bitmap at position (x, y)
        let _unused = (x, y, bitmap, width, height, stride);
    }

    /// Draws a rectangle at a specified position with the specified width, height, and color.
    pub fn draw_rect(&self, x: u32, y: u32, width: u32, height: u32, color: u16) {
        // Draw a rectangle at position (x, y) with the specified width and height
        let _unused = (x, y, width, height, color);
    }

    /// Partially refreshes the display.
    ///
    /// This function needs to be called periodically to refresh the display
    /// and only refreshes some of  the rows that are marked as dirty. It needs to be called
    /// multiple times to refresh the entire display.
    ///
    pub fn refresh(&self) {
        // Refresh the display
        let _unused = (&self.vram, &self.dirty);
    }
}
