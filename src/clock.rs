use core::ptr;

#[repr(C)]
pub struct FlashReg {
    pub acr: u32,
    pub keyr: u32,
    pub optkeyr: u32,
    pub sr: u32,
    pub cr: u32,
    pub optcr: u32,
}

pub const FLASH: *mut FlashReg = 0x4002_3C00 as *mut FlashReg;

#[repr(C)]
pub struct RccReg {
    pub cr: u32,
    pub pllcfgr: u32,
    pub cfgr: u32,
    pub cir: u32,

    pub ahb1rstr: u32,
    pub ahb2rstr: u32,
    pub ahb3rstr: u32,
    reserved1: u32,

    pub apb1_enr: u32,
    pub apb12enr: u32,
    reserved2: u32,
    reserved3: u32,

    pub ahb1enr: u32,
    pub ahb2enr: u32,
    pub ahb3enr: u32,
    reserved4: u32,

    pub apb1enr: u32,
    pub apb2enr: u32,
    reserved5: u32,
    reserved6: u32,

    pub ahb1lpenr: u32,
    pub ahb2lpenr: u32,
    pub ahb3lpenr: u32,
    reserved7: u32,

    pub apb1lpenr: u32,
    pub apb2lpenr: u32,
    reserved8: u32,
    reserved9: u32,

    pub bdcr: u32,
    pub csr: u32,
    reserved10: u32,
    reserved11: u32,

    sscgr: u32,
    pli2scfgr: u32,
}

pub const RCC: *mut RccReg = 0x4002_3800 as *mut RccReg;

macro_rules! write_bits {
    ($x:ident.$y:ident, $mask:expr, $data:expr  ) => {
        unsafe {
            let addr = ptr::addr_of_mut!((*$x).$y);
            let mut v: u32 = ptr::read_volatile(addr);
            v &= !$mask;
            v |= $data;
            ptr::write_volatile(addr, v);
        }
    };
}

pub fn init() {
    //#[cfg(feature = "none")]
    {
        // setup main PLL timing for external HSE
        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        #[cfg(feature = "brd-hactar-10")]
        let pll_m: u32 = 12;
        #[cfg(feature = "brd-blink-clk-a")]
        let pll_m: u32 = 8;
        let pll_n: u32 = 168;
        let pll_q: u32 = 4;

        mask |= 0b1 << 22;
        val |= 0b1 << 22; // select HSE
        mask |= 0b11 << 16;
        val |= 0b00 << 16; // set main division factor to 2

        assert!(pll_q >= 2);
        assert!(pll_q <= 0xF);
        assert!(pll_n >= 50);
        assert!(pll_n <= 432);
        assert!(pll_m >= 2);
        assert!(pll_m <= 63);

        mask |= 0b1111 << 24;
        val |= pll_q << 24;

        mask |= 0x7FFF << 0;
        val |= pll_n << 6;
        val |= pll_m;

        write_bits!(RCC.pllcfgr, mask, val);

        // setup flash wait states
        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        // set latency to 5 wait states - NOTE, if voltage is changed, need to change this
        mask |= 0b111 << 0;
        val |= 0b101 << 0;

        // enable data, instruction, prefetch cache
        mask |= 0b111 << 8;
        val |= 0b111 << 8;

        write_bits!(FLASH.acr, mask, val);

        // setup clock usage and dividers
        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        mask |= 0b11 << 0;
        val |= 0b10 << 0; // switch clock to PLL

        mask |= 0b1111 << 4;
        val |= 0b0000 << 4; // AHB Clk Div = 1

        mask |= 0b111 << 10;
        val |= 0b100 << 10; // APB1 CLk Div = 2

        mask |= 0b111 << 13;
        val |= 0b101 << 13; // APB2 Clk Div = 4

        write_bits!(RCC.cfgr, mask, val);
    }

    {
        // enable clocks for GPIO A,B,C

        let mut val: u32 = 0;
        let mut mask: u32 = 0;

        mask |= 0b111 << 0;
        val |= 0b111 << 0; // switch clock to PLL

        write_bits!(RCC.ahb1enr, mask, val);
    }
}
