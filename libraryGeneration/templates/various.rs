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