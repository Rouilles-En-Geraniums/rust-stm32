extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;


const RCC_ADR : u32 = 0x40023800;
        
const RCC_CR_OFFSET : u32 = 0x00;
const RCC_PLLCFGR_OFFSET : u32 = 0x04;
const RCC_CFGR_OFFSET : u32 = 0x08;
const RCC_CIR_OFFSET : u32 = 0x0C;
const RCC_AHB1RSTR_OFFSET : u32 = 0x10;
const RCC_AHB2RSTR_OFFSET : u32 = 0x14;
const RCC_AHB3RSTR_OFFSET : u32 = 0x18;
const RCC_APB1RSTR_OFFSET : u32 = 0x20;
const RCC_APB2RSTR_OFFSET : u32 = 0x24;
const RCC_AHB1ENR_OFFSET : u32 = 0x30;
const RCC_AHB2ENR_OFFSET : u32 = 0x34;
const RCC_AHB3ENR_OFFSET : u32 = 0x38;
const RCC_APB1ENR_OFFSET : u32 = 0x40;
const RCC_APB2ENR_OFFSET : u32 = 0x44;
const RCC_AHB1LPENR_OFFSET : u32 = 0x50;
const RCC_AHB2LPENR_OFFSET : u32 = 0x54;
const RCC_AHB3LPENR_OFFSET : u32 = 0x58;
const RCC_APB1LPENR_OFFSET : u32 = 0x60;
const RCC_APB2LPENR_OFFSET : u32 = 0x64;
const RCC_BDCR_OFFSET : u32 = 0x70;
const RCC_CSR_OFFSET : u32 = 0x74;
const RCC_SSCGR_OFFSET : u32 = 0x80;
const RCC_PLLISCFGR_OFFSET : u32 = 0x84;
    
pub const RCC_CR_HSION : u32 = 1 << 0;
pub const RCC_CR_HSIRDY : u32 = 1 << 1;
pub const RCC_CR_HSITRIM : u32 = 1 << 3;
pub const RCC_CR_HSICAL : u32 = 1 << 8;
pub const RCC_CR_HSEON : u32 = 1 << 16;
pub const RCC_CR_HSERDY : u32 = 1 << 17;
pub const RCC_CR_HSEBYP : u32 = 1 << 18;
pub const RCC_CR_CSSON : u32 = 1 << 19;
pub const RCC_CR_PLLON : u32 = 1 << 24;
pub const RCC_CR_PLLRDY : u32 = 1 << 25;
pub const RCC_CR_PLLI2SON : u32 = 1 << 26;
pub const RCC_CR_PLLI2SRDY : u32 = 1 << 27;
pub const RCC_CR_PLLSAION : u32 = 1 << 28;
pub const RCC_CR_PLLSAIRDY : u32 = 1 << 29;
pub const RCC_PLLCFGR_PLLM : u32 = 1 << 0;
pub const RCC_PLLCFGR_PLLN : u32 = 1 << 6;
pub const RCC_PLLCFGR_PLLP : u32 = 1 << 16;
pub const RCC_PLLCFGR_PLLSRC : u32 = 1 << 22;
pub const RCC_PLLCFGR_PLLQ : u32 = 1 << 24;
pub const RCC_CFGR_SW : u32 = 1 << 0;
pub const RCC_CFGR_SWS : u32 = 1 << 2;
pub const RCC_CFGR_HPRE : u32 = 1 << 4;
pub const RCC_CFGR_PPRE1 : u32 = 1 << 10;
pub const RCC_CFGR_PPRE2  : u32 = 1 << 13;
pub const RCC_CFGR_RTCPRE : u32 = 1 << 16;
pub const RCC_CFGR_MCO1 : u32 = 1 << 21;
pub const RCC_CFGR_I2SSCR : u32 = 1 << 23;
pub const RCC_CFGR_MCO1PRE : u32 = 1 << 24;
pub const RCC_CFGR_MCO2PRE : u32 = 1 << 27;
pub const RCC_CFGR_MCO2 : u32 = 1 << 30;
pub const RCC_CIR_CSSC : u32 = 1 << 23;
pub const RCC_CIR_PLLSAIRDYC : u32 = 1 << 22;
pub const RCC_CIR_PLLI2SRDYC : u32 = 1 << 21;
pub const RCC_CIR_PLLRDYC : u32 = 1 << 20;
pub const RCC_CIR_HSERDYC : u32 = 1 << 19;
pub const RCC_CIR_HSIRDYC : u32 = 1 << 18;
pub const RCC_CIR_LSERDYC : u32 = 1 << 17;
pub const RCC_CIR_LSIRDYC : u32 = 1 << 16;
pub const RCC_CIR_PLLSAIRDYIE : u32 = 1 << 14;
pub const RCC_CIR_PLLI2SRDYIE : u32 = 1 << 13;
pub const RCC_CIR_PLLRDYIE : u32 = 1 << 12;
pub const RCC_CIR_HSERDYIE : u32 = 1 << 11;
pub const RCC_CIR_HSIRDYIE : u32 = 1 << 10;
pub const RCC_CIR_LSERDYIE : u32 = 1 << 9;
pub const RCC_CIR_LSIRDYIE : u32 = 1 << 8;
pub const RCC_CIR_CSSF : u32 = 1 << 7;
pub const RCC_CIR_PLLSAIRDYF : u32 = 1 << 6;
pub const RCC_CIR_PLLI2SRDYF : u32 = 1 << 5;
pub const RCC_CIR_PLLRDYF : u32 = 1 << 4;
pub const RCC_CIR_HSERDYF : u32 = 1 << 3;
pub const RCC_CIR_HSIRDYF : u32 = 1 << 2;
pub const RCC_CIR_LSERDYF : u32 = 1 << 1;
pub const RCC_CIR_LSIRDYF : u32 = 1 << 0;
pub const RCC_AHB1RSTR_GPIOARST : u32 = 1 << 0;
pub const RCC_AHB1RSTR_GPIOBRST : u32 = 1 << 1;
pub const RCC_AHB1RSTR_GPIOCRST : u32 = 1 << 2;
pub const RCC_AHB1RSTR_GPIODRST : u32 = 1 << 3;
pub const RCC_AHB1RSTR_GPIOERST : u32 = 1 << 4;
pub const RCC_AHB1RSTR_GPIOFRST : u32 = 1 << 5;
pub const RCC_AHB1RSTR_GPIOGRST : u32 = 1 << 6;
pub const RCC_AHB1RSTR_GPIOHRST : u32 = 1 << 7;
pub const RCC_AHB1RSTR_GPIOIRST : u32 = 1 << 8;
pub const RCC_AHB1RSTR_GPIOJRST : u32 = 1 << 9;
pub const RCC_AHB1RSTR_GPIOKRST : u32 = 1 << 10;
pub const RCC_APB1RSTR_TIM2RST : u32 = 1 << 0;
pub const RCC_APB1RSTR_TIM3RST : u32 = 1 << 1;
pub const RCC_APB1RSTR_TIM4RST : u32 = 1 << 2;
pub const RCC_APB1RSTR_TIM5RST : u32 = 1 << 3;
pub const RCC_APB1RSTR_TIM6RST : u32 = 1 << 4;
pub const RCC_APB1RSTR_TIM7RST : u32 = 1 << 5;
pub const RCC_APB1RSTR_TIM12RST : u32 = 1 << 6;
pub const RCC_APB1RSTR_TIM13RST : u32 = 1 << 7;
pub const RCC_APB1RSTR_TIM14RST : u32 = 1 << 8;
pub const RCC_APB2RSTR_TIM1RST : u32 = 1 << 0;
pub const RCC_APB2RSTR_TIM8RST : u32 = 1 << 1;
pub const RCC_APB2RSTR_TIM9RST : u32 = 1 << 16;
pub const RCC_APB2RSTR_TIM10RST : u32 = 1 << 17;
pub const RCC_APB2RSTR_TIM11RST : u32 = 1 << 18;
pub const RCC_AHB1ENR_GPIOAEN : u32 = 1 << 0;
pub const RCC_AHB1ENR_GPIOBEN : u32 = 1 << 1;
pub const RCC_AHB1ENR_GPIOCEN : u32 = 1 << 2;
pub const RCC_AHB1ENR_GPIODEN : u32 = 1 << 3;
pub const RCC_AHB1ENR_GPIOEEN : u32 = 1 << 4;
pub const RCC_AHB1ENR_GPIOFEN : u32 = 1 << 5;
pub const RCC_AHB1ENR_GPIOGEN : u32 = 1 << 6;
pub const RCC_AHB1ENR_GPIOHEN : u32 = 1 << 7;
pub const RCC_AHB1ENR_GPIOIEN : u32 = 1 << 8;
pub const RCC_AHB1ENR_GPIOJEN : u32 = 1 << 9;
pub const RCC_AHB1ENR_GPIOKEN : u32 = 1 << 10;
pub const RCC_APB1ENR_TIM2EN : u32 = 1 << 0;
pub const RCC_APB1ENR_TIM3EN : u32 = 1 << 1;
pub const RCC_APB1ENR_TIM4EN : u32 = 1 << 2;
pub const RCC_APB1ENR_TIM5EN : u32 = 1 << 3;
pub const RCC_APB1ENR_TIM6EN : u32 = 1 << 4;
pub const RCC_APB1ENR_TIM7EN : u32 = 1 << 5;
pub const RCC_APB1ENR_TIM12EN : u32 = 1 << 6;
pub const RCC_APB1ENR_TIM13EN : u32 = 1 << 7;
pub const RCC_APB1ENR_TIM14EN : u32 = 1 << 8;
pub const RCC_APB2ENR_TIM1EN : u32 = 1 << 0;
pub const RCC_APB2ENR_TIM8EN : u32 = 1 << 1;
pub const RCC_APB2ENR_TIM9EN : u32 = 1 << 16;
pub const RCC_APB2ENR_TIM10EN : u32 = 1 << 17;
pub const RCC_APB2ENR_TIM11EN : u32 = 1 << 18;
pub const RCC_BDCR_BDRST : u32 = 1 << 16;
pub const RCC_BDCR_RTCEN : u32 = 1 << 15;
pub const RCC_BDCR_RTCSEL : u32 = 1 << 8;
pub const RCC_BDCR_LSEBYP : u32 = 1 << 2;
pub const RCC_BDCR_LSERDY : u32 = 1 << 1;
pub const RCC_BDCR_LSEON : u32 = 1 << 0;
pub const RCC_CSR_LPWRRSTF : u32 = 1 << 31;
pub const RCC_CSR_WWDGRSTF : u32 = 1 << 30;
pub const RCC_CSR_WDGRSTF : u32 = 1 << 29;
pub const RCC_CSR_SFTRSTF : u32 = 1 << 28;
pub const RCC_CSR_PORRSTF : u32 = 1 << 27;
pub const RCC_CSR_PADRSTF : u32 = 1 << 26;
pub const RCC_CSR_BORRSTF : u32 = 1 << 25;
pub const RCC_CSR_RMVF : u32 = 1 << 24;
pub const RCC_CSR_LSIRDY : u32 = 1 << 1;
pub const RCC_CSR_LSION : u32 = 1 << 0;
pub const RCC_SSCGR_SSCGEN : u32 = 1 << 31;
pub const RCC_SSCGR_SPREADSEL : u32 = 1 << 30;
pub const RCC_SSCGR_INCSTEP : u32 = 1 << 13;
pub const RCC_SSCGR_MODPER : u32 = 1 << 0;
pub const RCC_PPLI2SCFGR_PLLI2SR2 : u32 = 1 << 30;
pub const RCC_PPLI2SCFGR_PLLI2SR1 : u32 = 1 << 29;
pub const RCC_PPLI2SCFGR_PLLI2SR0 : u32 = 1 << 28;
pub const RCC_PPLI2SCFGR_PLLI2SQ : u32 = 1 << 24;
pub const RCC_PPLI2SCFGR_PLLI2SN : u32 = 1 << 6;
pub const RCC_PLLSAICFGR_PLLSAIR : u32 = 1 << 28;
pub const RCC_PLLSAICFGR_PLLSAIQ : u32 = 1 << 24;
pub const RCC_PLLSAICFGR_PLLSAIN : u32 = 1 << 6;
pub const RCC_DCKCFGR_TIMPRE : u32 = 1 << 24;
pub const RCC_DCKCFGR_SAI1BSCR : u32 = 1 << 22;
pub const RCC_DCKCFGR_SAI1ASCR : u32 = 1 << 20;
pub const RCC_DCKCFGR_PLLSAIDIVR : u32 = 1 << 16;
pub const RCC_DCKCFGR_PLLSAIDIVQ : u32 = 1 << 8;
pub const RCC_DCKCFGR_PLLI2SDIVQ : u32 = 1 << 0;
        
pub fn rcc_cr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_CR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_pllcfgr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_PLLCFGR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_cfgr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_CFGR_OFFSET) as *mut u32, value)
    };
}

pub fn rcc_ahb1rstr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB1RSTR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb2rstr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB2RSTR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb3rstr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB3RSTR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_apb1rstr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_APB1RSTR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_apb2rstr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_APB2RSTR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb1enr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB1ENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb2enr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB2ENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb3enr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB3ENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_apb1enr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_APB1ENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_apb2enr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_APB2ENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb1lpenr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB1LPENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb2lpenr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB2LPENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_ahb3lpenr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_AHB3LPENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_apb1lpenr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_APB1LPENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_apb2lpenr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_APB2LPENR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_bdcr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_BDCR_OFFSET) as *mut u32, value)
    };
}

pub fn rcc_sscgr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_SSCGR_OFFSET) as *mut u32, value)
    };
}
pub fn rcc_plliscfgr_write(value: u32) {
    unsafe {
        write_volatile( (RCC_ADR + RCC_PLLISCFGR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn rcc_cr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_CR_OFFSET) as *mut u32)
    }
}
pub fn rcc_pllcfgr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_PLLCFGR_OFFSET) as *mut u32)
    }
}
pub fn rcc_cfgr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_CFGR_OFFSET) as *mut u32)
    }
}
pub fn rcc_cir_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_CIR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb1rstr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB1RSTR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb2rstr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB2RSTR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb3rstr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB3RSTR_OFFSET) as *mut u32)
    }
}
pub fn rcc_apb1rstr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_APB1RSTR_OFFSET) as *mut u32)
    }
}
pub fn rcc_apb2rstr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_APB2RSTR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb1enr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB1ENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb2enr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB2ENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb3enr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB3ENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_apb1enr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_APB1ENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_apb2enr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_APB2ENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb1lpenr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB1LPENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb2lpenr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB2LPENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_ahb3lpenr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_AHB3LPENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_apb1lpenr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_APB1LPENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_apb2lpenr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_APB2LPENR_OFFSET) as *mut u32)
    }
}
pub fn rcc_bdcr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_BDCR_OFFSET) as *mut u32)
    }
}
pub fn rcc_csr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_CSR_OFFSET) as *mut u32)
    }
}
pub fn rcc_sscgr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_SSCGR_OFFSET) as *mut u32)
    }
}
pub fn rcc_plliscfgr_read() -> u32 {
    unsafe {
        read_volatile( (RCC_ADR + RCC_PLLISCFGR_OFFSET) as *mut u32)
    }
}
    



//fn initRegister(name: u8){
    //RCC_AHB1ENR |= RCC_GPIODEN;
//}
