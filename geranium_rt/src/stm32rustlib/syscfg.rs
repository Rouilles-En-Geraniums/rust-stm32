extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;


const SYSCFG_ADR : u32 = 0x40013800;
        
const SYSCFG_MEMRMP_OFFSET : u32 = 0x00;
const SYSCFG_PMC_OFFSET : u32 = 0x04;
const SYSCFG_EXTICR1_OFFSET : u32 = 0x08;
const SYSCFG_EXTICR2_OFFSET : u32 = 0x0C;
const SYSCFG_EXTICR3_OFFSET : u32 = 0x10;
const SYSCFG_EXTICR4_OFFSET : u32 = 0x14;
const SYSCFG_CMPCR_OFFSET : u32 = 0x20;
    
pub const SYSCFG_MEMRMP_MEM_MODE : u32 = 1 << 0;
pub const SYSCFG_PMC_MII_RMII_SEL : u32 = 1 << 23;
pub const SYSCFG_EXTICR1_EXTI0 : u32 = 1 << 0;
pub const SYSCFG_EXTICR1_EXTI1 : u32 = 1 << 4;
pub const SYSCFG_EXTICR1_EXTI2 : u32 = 1 << 8;
pub const SYSCFG_EXTICR1_EXTI3 : u32 = 1 << 12;
pub const SYSCFG_EXTICR2_EXTI4 : u32 = 1 << 0;
pub const SYSCFG_EXTICR2_EXTI5 : u32 = 1 << 4;
pub const SYSCFG_EXTICR2_EXTI6 : u32 = 1 << 8;
pub const SYSCFG_EXTICR2_EXTI7 : u32 = 1 << 12;
pub const SYSCFG_EXTICR3_EXTI8 : u32 = 1 << 0;
pub const SYSCFG_EXTICR3_EXTI9 : u32 = 1 << 4;
pub const SYSCFG_EXTICR3_EXTI10 : u32 = 1 << 8;
pub const SYSCFG_EXTICR3_EXTI11 : u32 = 1 << 12;
pub const SYSCFG_EXTICR4_EXTI12 : u32 = 1 << 0;
pub const SYSCFG_EXTICR4_EXTI13 : u32 = 1 << 4;
pub const SYSCFG_EXTICR4_EXTI14 : u32 = 1 << 8;
pub const SYSCFG_EXTICR4_EXTI15 : u32 = 1 << 12;
pub const SYSCFG_CMPCR_CMP_PD : u32 = 1 << 0;
pub const SYSCFG_CMPCR_READY : u32 = 1 << 8;
        
pub fn syscfg_memrmp_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_MEMRMP_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_pmc_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_PMC_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_exticr1_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_EXTICR1_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_exticr2_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_EXTICR2_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_exticr3_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_EXTICR3_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_exticr4_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_EXTICR4_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_cmpcr_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_CMPCR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn syscfg_memrmp_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_MEMRMP_OFFSET) as *mut u32)
    }
}
pub fn syscfg_pmc_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_PMC_OFFSET) as *mut u32)
    }
}
pub fn syscfg_exticr1_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_EXTICR1_OFFSET) as *mut u32)
    }
}
pub fn syscfg_exticr2_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_EXTICR2_OFFSET) as *mut u32)
    }
}
pub fn syscfg_exticr3_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_EXTICR3_OFFSET) as *mut u32)
    }
}
pub fn syscfg_exticr4_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_EXTICR4_OFFSET) as *mut u32)
    }
}
pub fn syscfg_cmpcr_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_CMPCR_OFFSET) as *mut u32)
    }
}
    