extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;

pub const HIGH: u8 = 1;
pub const LOW: u8 = 0;
pub const MODER: u8 = 0;
pub const PUPDR: u8 = 1;
pub const OSPEEDER: u8 = 2;
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