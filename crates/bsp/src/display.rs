extern crate hal;

#[cfg(feature = "std")]
extern crate std;

const WIDTH: usize = 128;
const HEIGHT: usize = 64;

pub struct Display {
    vram: [u16; WIDTH * HEIGHT],
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

    pub fn size(&self) -> (u32, u32) {
        (WIDTH as u32, HEIGHT as u32)
    }

    pub fn draw_pixel(&self, x: u32, y: u32, color: u16 ) {
        // Draw a pixel at position (x, y)
        let _unused = (x, y, color);
    }

    pub fn draw_bitmap(&self, x: u32, y: u32, bitmap: &[u16] ,
                       width: u32, height: u32, stride: u32) {
        // Draw a bitmap at position (x, y)
        let _unused = (x, y, bitmap, width, height, stride);
    }

    pub fn draw_rect(&self, x: u32, y: u32,
                     width: u32, height: u32, color: u16) {
        // Draw a rectangle at position (x, y) with the specified width and height
        let _unused = (x, y, width, height, color);
    }

    pub fn refresh(&self) {
        // Refresh the display
        let _unused = ( &self.vram, &self.dirty );
    }

}
