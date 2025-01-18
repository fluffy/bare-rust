// DO NOT EDIT. This was generated by svd2rusty
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
pub mod RCC {
    pub mod cr {
        pub const HSION: u8 = 0;
        pub const HSIRDY: u8 = 1;
        pub const HSITRIM: u8 = 3;
        pub const HSICAL: u8 = 8;
        pub const HSEON: u8 = 16;
        pub const HSERDY: u8 = 17;
        pub const HSEBYP: u8 = 18;
        pub const CSSON: u8 = 19;
        pub const PLLON: u8 = 24;
        pub const PLLRDY: u8 = 25;
    }
    pub mod cfgr {
        pub const SW: u8 = 0;
        pub const SWS: u8 = 2;
        pub const HPRE: u8 = 4;
        pub const PPRE: u8 = 8;
        pub const ADCPRE: u8 = 14;
        pub const PLLSRC: u8 = 15;
        pub const PLLXTPRE: u8 = 17;
        pub const PLLMUL: u8 = 18;
        pub const MCO: u8 = 24;
        pub const MCOPRE: u8 = 28;
        pub const PLLNODIV: u8 = 31;
    }
    pub mod cir {
        pub const LSIRDYF: u8 = 0;
        pub const LSERDYF: u8 = 1;
        pub const HSIRDYF: u8 = 2;
        pub const HSERDYF: u8 = 3;
        pub const PLLRDYF: u8 = 4;
        pub const HSI14RDYF: u8 = 5;
        pub const HSI48RDYF: u8 = 6;
        pub const CSSF: u8 = 7;
        pub const LSIRDYIE: u8 = 8;
        pub const LSERDYIE: u8 = 9;
        pub const HSIRDYIE: u8 = 10;
        pub const HSERDYIE: u8 = 11;
        pub const PLLRDYIE: u8 = 12;
        pub const HSI14RDYE: u8 = 13;
        pub const HSI48RDYIE: u8 = 14;
        pub const LSIRDYC: u8 = 16;
        pub const LSERDYC: u8 = 17;
        pub const HSIRDYC: u8 = 18;
        pub const HSERDYC: u8 = 19;
        pub const PLLRDYC: u8 = 20;
        pub const HSI14RDYC: u8 = 21;
        pub const HSI48RDYC: u8 = 22;
        pub const CSSC: u8 = 23;
    }
    pub mod apb2rstr {
        pub const SYSCFGRST: u8 = 0;
        pub const ADCRST: u8 = 9;
        pub const TIM1RST: u8 = 11;
        pub const SPI1RST: u8 = 12;
        pub const USART1RST: u8 = 14;
        pub const TIM15RST: u8 = 16;
        pub const TIM16RST: u8 = 17;
        pub const TIM17RST: u8 = 18;
        pub const DBGMCURST: u8 = 22;
    }
    pub mod apb1rstr {
        pub const TIM2RST: u8 = 0;
        pub const TIM3RST: u8 = 1;
        pub const TIM6RST: u8 = 4;
        pub const TIM7RST: u8 = 5;
        pub const TIM14RST: u8 = 8;
        pub const WWDGRST: u8 = 11;
        pub const SPI2RST: u8 = 14;
        pub const USART2RST: u8 = 17;
        pub const USART3RST: u8 = 18;
        pub const USART4RST: u8 = 19;
        pub const I2C1RST: u8 = 21;
        pub const I2C2RST: u8 = 22;
        pub const USBRST: u8 = 23;
        pub const CANRST: u8 = 25;
        pub const CRSRST: u8 = 27;
        pub const PWRRST: u8 = 28;
        pub const DACRST: u8 = 29;
        pub const CECRST: u8 = 30;
    }
    pub mod ahbenr {
        pub const DMA1EN: u8 = 0;
        pub const SRAMEN: u8 = 2;
        pub const FLITFEN: u8 = 4;
        pub const CRCEN: u8 = 6;
        pub const IOPAEN: u8 = 17;
        pub const IOPBEN: u8 = 18;
        pub const IOPCEN: u8 = 19;
        pub const IOPDEN: u8 = 20;
        pub const IOPFEN: u8 = 22;
        pub const TSCEN: u8 = 24;
    }
    pub mod apb2enr {
        pub const SYSCFGEN: u8 = 0;
        pub const ADCEN: u8 = 9;
        pub const TIM1EN: u8 = 11;
        pub const SPI1EN: u8 = 12;
        pub const USART1EN: u8 = 14;
        pub const TIM15EN: u8 = 16;
        pub const TIM16EN: u8 = 17;
        pub const TIM17EN: u8 = 18;
        pub const DBGMCUEN: u8 = 22;
    }
    pub mod apb1enr {
        pub const TIM2EN: u8 = 0;
        pub const TIM3EN: u8 = 1;
        pub const TIM6EN: u8 = 4;
        pub const TIM7EN: u8 = 5;
        pub const TIM14EN: u8 = 8;
        pub const WWDGEN: u8 = 11;
        pub const SPI2EN: u8 = 14;
        pub const USART2EN: u8 = 17;
        pub const USART3EN: u8 = 18;
        pub const USART4EN: u8 = 19;
        pub const I2C1EN: u8 = 21;
        pub const I2C2EN: u8 = 22;
        pub const USBRST: u8 = 23;
        pub const CANEN: u8 = 25;
        pub const CRSEN: u8 = 27;
        pub const PWREN: u8 = 28;
        pub const DACEN: u8 = 29;
        pub const CECEN: u8 = 30;
    }
    pub mod bdcr {
        pub const LSEON: u8 = 0;
        pub const LSERDY: u8 = 1;
        pub const LSEBYP: u8 = 2;
        pub const LSEDRV: u8 = 3;
        pub const RTCSEL: u8 = 8;
        pub const RTCEN: u8 = 15;
        pub const BDRST: u8 = 16;
    }
    pub mod csr {
        pub const LSION: u8 = 0;
        pub const LSIRDY: u8 = 1;
        pub const RMVF: u8 = 24;
        pub const OBLRSTF: u8 = 25;
        pub const PINRSTF: u8 = 26;
        pub const PORRSTF: u8 = 27;
        pub const SFTRSTF: u8 = 28;
        pub const IWDGRSTF: u8 = 29;
        pub const WWDGRSTF: u8 = 30;
        pub const LPWRRSTF: u8 = 31;
    }
    pub mod ahbrstr {
        pub const IOPARST: u8 = 17;
        pub const IOPBRST: u8 = 18;
        pub const IOPCRST: u8 = 19;
        pub const IOPDRST: u8 = 20;
        pub const IOPFRST: u8 = 22;
        pub const TSCRST: u8 = 24;
    }
    pub mod cfgr2 {
        pub const PREDIV: u8 = 0;
    }
    pub mod cfgr3 {
        pub const USART1SW: u8 = 0;
        pub const I2C1SW: u8 = 4;
        pub const CECSW: u8 = 6;
        pub const USBSW: u8 = 7;
        pub const ADCSW: u8 = 8;
        pub const USART2SW: u8 = 16;
    }
    pub mod cr2 {
        pub const HSI14ON: u8 = 0;
        pub const HSI14RDY: u8 = 1;
        pub const HSI14DIS: u8 = 2;
        pub const HSI14TRIM: u8 = 3;
        pub const HSI14CAL: u8 = 8;
        pub const HSI48ON: u8 = 16;
        pub const HSI48RDY: u8 = 17;
        pub const HSI48CAL: u8 = 24;
    }
}
pub mod USART {
    pub mod cr1 {
        pub const UE: u8 = 0;
        pub const UESM: u8 = 1;
        pub const RE: u8 = 2;
        pub const TE: u8 = 3;
        pub const IDLEIE: u8 = 4;
        pub const RXNEIE: u8 = 5;
        pub const TCIE: u8 = 6;
        pub const TXEIE: u8 = 7;
        pub const PEIE: u8 = 8;
        pub const PS: u8 = 9;
        pub const PCE: u8 = 10;
        pub const WAKE: u8 = 11;
        pub const M: u8 = 12;
        pub const MME: u8 = 13;
        pub const CMIE: u8 = 14;
        pub const OVER8: u8 = 15;
        pub const DEDT: u8 = 16;
        pub const DEAT: u8 = 21;
        pub const RTOIE: u8 = 26;
        pub const EOBIE: u8 = 27;
        pub const M1: u8 = 28;
    }
    pub mod cr2 {
        pub const ADD4: u8 = 28;
        pub const ADD0: u8 = 24;
        pub const RTOEN: u8 = 23;
        pub const ABRMOD: u8 = 21;
        pub const ABREN: u8 = 20;
        pub const MSBFIRST: u8 = 19;
        pub const DATAINV: u8 = 18;
        pub const TXINV: u8 = 17;
        pub const RXINV: u8 = 16;
        pub const SWAP: u8 = 15;
        pub const LINEN: u8 = 14;
        pub const STOP: u8 = 12;
        pub const CLKEN: u8 = 11;
        pub const CPOL: u8 = 10;
        pub const CPHA: u8 = 9;
        pub const LBCL: u8 = 8;
        pub const LBDIE: u8 = 6;
        pub const LBDL: u8 = 5;
        pub const ADDM7: u8 = 4;
    }
    pub mod cr3 {
        pub const WUFIE: u8 = 22;
        pub const WUS: u8 = 20;
        pub const SCARCNT: u8 = 17;
        pub const DEP: u8 = 15;
        pub const DEM: u8 = 14;
        pub const DDRE: u8 = 13;
        pub const OVRDIS: u8 = 12;
        pub const ONEBIT: u8 = 11;
        pub const CTSIE: u8 = 10;
        pub const CTSE: u8 = 9;
        pub const RTSE: u8 = 8;
        pub const DMAT: u8 = 7;
        pub const DMAR: u8 = 6;
        pub const SCEN: u8 = 5;
        pub const NACK: u8 = 4;
        pub const HDSEL: u8 = 3;
        pub const IRLP: u8 = 2;
        pub const IREN: u8 = 1;
        pub const EIE: u8 = 0;
    }
    pub mod brr {
        pub const DIV_Mantissa: u8 = 4;
        pub const DIV_Fraction: u8 = 0;
    }
    pub mod gtpr {
        pub const GT: u8 = 8;
        pub const PSC: u8 = 0;
    }
    pub mod rtor {
        pub const BLEN: u8 = 24;
        pub const RTO: u8 = 0;
    }
    pub mod rqr {
        pub const TXFRQ: u8 = 4;
        pub const RXFRQ: u8 = 3;
        pub const MMRQ: u8 = 2;
        pub const SBKRQ: u8 = 1;
        pub const ABRRQ: u8 = 0;
    }
    pub mod isr {
        pub const REACK: u8 = 22;
        pub const TEACK: u8 = 21;
        pub const WUF: u8 = 20;
        pub const RWU: u8 = 19;
        pub const SBKF: u8 = 18;
        pub const CMF: u8 = 17;
        pub const BUSY: u8 = 16;
        pub const ABRF: u8 = 15;
        pub const ABRE: u8 = 14;
        pub const EOBF: u8 = 12;
        pub const RTOF: u8 = 11;
        pub const CTS: u8 = 10;
        pub const CTSIF: u8 = 9;
        pub const LBDF: u8 = 8;
        pub const TXE: u8 = 7;
        pub const TC: u8 = 6;
        pub const RXNE: u8 = 5;
        pub const IDLE: u8 = 4;
        pub const ORE: u8 = 3;
        pub const NF: u8 = 2;
        pub const FE: u8 = 1;
        pub const PE: u8 = 0;
    }
    pub mod icr {
        pub const WUCF: u8 = 20;
        pub const CMCF: u8 = 17;
        pub const EOBCF: u8 = 12;
        pub const RTOCF: u8 = 11;
        pub const CTSCF: u8 = 9;
        pub const LBDCF: u8 = 8;
        pub const TCCF: u8 = 6;
        pub const IDLECF: u8 = 4;
        pub const ORECF: u8 = 3;
        pub const NCF: u8 = 2;
        pub const FECF: u8 = 1;
        pub const PECF: u8 = 0;
    }
    pub mod rdr {
        pub const RDR: u8 = 0;
    }
    pub mod tdr {
        pub const TDR: u8 = 0;
    }
}
pub mod FLASH {
    pub mod acr {
        pub const LATENCY: u8 = 0;
        pub const PRFTBE: u8 = 4;
        pub const PRFTBS: u8 = 5;
    }
    pub mod keyr {
        pub const FKEYR: u8 = 0;
    }
    pub mod optkeyr {
        pub const OPTKEYR: u8 = 0;
    }
    pub mod sr {
        pub const EOP: u8 = 5;
        pub const WRPRT: u8 = 4;
        pub const PGERR: u8 = 2;
        pub const BSY: u8 = 0;
    }
    pub mod cr {
        pub const FORCE_OPTLOAD: u8 = 13;
        pub const EOPIE: u8 = 12;
        pub const ERRIE: u8 = 10;
        pub const OPTWRE: u8 = 9;
        pub const LOCK: u8 = 7;
        pub const STRT: u8 = 6;
        pub const OPTER: u8 = 5;
        pub const OPTPG: u8 = 4;
        pub const MER: u8 = 2;
        pub const PER: u8 = 1;
        pub const PG: u8 = 0;
    }
    pub mod ar {
        pub const FAR: u8 = 0;
    }
    pub mod obr {
        pub const OPTERR: u8 = 0;
        pub const RDPRT: u8 = 1;
        pub const WDG_SW: u8 = 8;
        pub const nRST_STOP: u8 = 9;
        pub const nRST_STDBY: u8 = 10;
        pub const nBOOT0: u8 = 11;
        pub const nBOOT1: u8 = 12;
        pub const VDDA_MONITOR: u8 = 13;
        pub const RAM_PARITY_CHECK: u8 = 14;
        pub const BOOT_SEL: u8 = 15;
        pub const Data0: u8 = 16;
        pub const Data1: u8 = 24;
    }
    pub mod wrpr {
        pub const WRP: u8 = 0;
    }
}
