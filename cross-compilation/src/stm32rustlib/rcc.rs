extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;

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
    



fn initRegister(name: u8){
    //RCC_AHB1ENR |= RCC_GPIODEN;
}
