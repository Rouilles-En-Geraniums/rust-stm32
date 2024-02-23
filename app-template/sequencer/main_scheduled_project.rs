#![no_std]
#![no_main]
/*

/!\ WARNING
This is a scheduled project, meaning that the main.rs file is to be generated by running generate-main.py 
(with the scheduling you want as a JSON file (see rust-stm32/taskSequencing/README.md))

All content of this main.rs file will be overwritten when you run generate-main.py ; 
from your point of view the entry point is user_tasks.rs, as it's where the scheduled tasks will be taken from. 

*/

use geranium_rt::println;

#[no_mangle]
fn init(){
    
}

#[no_mangle]
fn main(){
    println!("Hello world (I am a placeholder)");
}
