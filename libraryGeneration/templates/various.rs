extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;

pub const HIGH: u8 = 1;
pub const LOW: u8 = 0;


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