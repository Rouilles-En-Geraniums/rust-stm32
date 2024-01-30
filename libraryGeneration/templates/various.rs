extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;

pub const HIGH: u8 = 1;
pub const LOW: u8 = 0;

fn mask(l: u32) -> u32 {
    (1 << (l))-1
}

fn get_bits(x: u32, i:u32, l:u32) -> u32 {
    ((x)>>(i)) & mask(l)
}

fn rep_bits(x: u32, i:u32, l:u32, y:u32) -> u32 {
    ((x)&!(mask(l)<<i))|((y)<<(i))
}

/*
fn wait(t: u32){
    // fonction blocante
    // t en ms

    //const ONE_SECOND: u32 = 30000000
    let n = t/1000 * ONE_SECOND;

    for i in 0..n {
        NOP; // TODO ? maybe it is a macro
    }
} */