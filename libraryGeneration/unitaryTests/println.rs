#![no_std]
#![no_main]

// The following creates are required to use println
extern crate geranium_rt;
extern crate core;
use geranium_rt::stm32rustlib::geranium_print::*;
use core::format_args;

// TODO you can impl Debug trait to use it
// it could also work with Display but it hasn't been tested
#[derive(Debug)]
struct TupleStruct(char, u32);

#[no_mangle]
fn main() {

    let my_led = ('D', 12); // Built-in green led
    let my_ledst = TupleStruct('A', 323_u32);

    // You must use the format_args! macro to println
    println(format_args!("my_led: ({},{})", my_led.0, my_led.1));
    println(format_args!("my_ledst: {my_ledst:?}"));

    loop {
    }
}
