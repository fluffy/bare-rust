

#[cfg(feature = "board-hactar12")]
pub mod info {
    pub const CLOCK_PLL_M: u32 = 12;
}

#[cfg(feature = "board-sim")]
pub mod info {
    pub const CLOCK_PLL_M: u32 = 12;
}

#[cfg(feature = "board-blinkA")]
pub mod info {
    pub const CLOCK_PLL_M: u32 = 8;
}
