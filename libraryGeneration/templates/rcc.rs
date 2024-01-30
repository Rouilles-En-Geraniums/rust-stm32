extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;
{% include "address.rs" %}


fn initRegister(name: u8){
    //RCC_AHB1ENR |= RCC_GPIODEN;
}

