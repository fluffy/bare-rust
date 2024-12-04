use core::ptr;
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

pub const RCC: *mut RccReg = 0x4002_1000 as *mut RccReg;

pub fn init() {
    unsafe {
        let addr = ptr::addr_of_mut!((*RCC).pllcfgr);
        let mut val: u32 = ptr::read_volatile(addr);

        let pll_m: u32 = 12;
        let pll_n: u32 = 168;
        let pll_q: u32 = 4;
        val |= 0x1 << 22 ; // select HSE
        val |= 0x00 << 16; // set main division factor to 2

        assert!(pll_q >= 2);
        assert!(pll_q <= 0xF);
        assert!(pll_n >= 50);
        assert!(pll_n <= 432);
        assert!(pll_m >= 2);
        assert!(pll_m <= 63);

        val |= pll_q << 24;
        val |= pll_n << 6;
        val |= pll_m;

        ptr::write_volatile(addr, val);
    }
}
