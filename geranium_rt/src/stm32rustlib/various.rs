extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;

pub const HIGH: u8 = 1;
pub const LOW: u8 = 0;

pub fn mask(l: u32) -> u32 {
    (1 << (l))-1
}

pub fn get_bits(x: u32, i:u32, l:u32) -> u32 {
    ((x)>>(i)) & mask(l)
}

pub fn rep_bits(x: u32, i:u32, l:u32, y:u32) -> u32 {
    ((x)&!(mask(l)<<i))|((y)<<(i))
}



// Some FLASH variables
pub const FLASH_ADR : u32 = 0x40023C00;
pub const FLASH_ACR_OFFSET : u32 = 0x00;
pub const FLASH_ACR_LATENCY : u32 =  0; //range = 2 bits
pub const FLASH_ACR_DCEN : u32 = 1 << 10;
pub const FLASH_ACR_ICEN : u32 = 1 << 9;
pub fn flash_acr_write(value: u32) {
    unsafe {
        write_volatile( (FLASH_ADR + FLASH_ACR_OFFSET) as *mut u32, value)
    };
}
pub fn flash_acr_read() -> u32 {
    unsafe {
        read_volatile( (FLASH_ADR + FLASH_ACR_OFFSET) as *mut u32)
    }
}


// Some SYSCFG variables
pub const SYSCFG_ADR :u32 = 0x40013800;
pub const SYSCFG_MEMRMP_OFFSET : u32 = 0x00;
pub fn syscfg_memrmp_write(value: u32) {
    unsafe {
        write_volatile( (SYSCFG_ADR + SYSCFG_MEMRMP_OFFSET) as *mut u32, value)
    };
}
pub fn syscfg_memrmp_read() -> u32 {
    unsafe {
        read_volatile( (SYSCFG_ADR + SYSCFG_MEMRMP_OFFSET) as *mut u32)
    }
}


// Some DEBUG variables
pub const DBG_DEMCR : u32 = 0xE000EDFC;
pub const DBG_DEMCR_TRCENA : u32 = 1 << 24;
pub fn dbg_demcr_write(value: u32) {
    unsafe {
        write_volatile( DBG_DEMCR as *mut u32, value)
    };
}
pub fn dbg_demcr_read() -> u32 {
    unsafe {
        read_volatile( DBG_DEMCR as *mut u32)
    }
}

// Some ITM variables
pub const ITM_TRACE_EN : u32 = 0xE0000E00;
pub const ITM_TRACE_EN_PORT0 : u32 = 1 << 0;
pub fn itm_trace_en_write(value: u32) {
    unsafe {
        write_volatile( ITM_TRACE_EN as *mut u32, value)
    };
}
pub fn itm_trace_en_read() -> u32 {
    unsafe {
        read_volatile( ITM_TRACE_EN as *mut u32)
    }
}