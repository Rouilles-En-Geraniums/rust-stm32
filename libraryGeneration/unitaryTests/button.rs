#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;

use crate::stm32rustlib::exti;
use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::tim;

pub mod stm32rustlib;

// bit range macros
//#define MASK(l)	((1 << (l))-1)
fn mask(l: u32) -> u32 {
    (1 << (l))-1
}

//#define GET_BITS(x, i, l)		(((x)>>(i))&MASK(l))
fn get_bits(x: u32, i:u32, l:u32) -> u32 {
    ((x)>>(i)) & mask(l)
}

//#define REP_BITS(x, i, l, y)	(((x)&~(MASK(l)<<i))|((y)<<(i)))
fn rep_bits(x: u32, i:u32, l:u32, y:u32) -> u32 {
    ((x)&!(mask(l)<<i))|((y)<<(i))
}

const GPIO_MODER_IN:u32	=   0b00;
const GPIO_MODER_OUT:u32 =	0b01;
const GPIO_MODER_ALT:u32 =	0b10;
const GPIO_MODER_ANA:u32 =	0b11;

const GPIO_OSPEEDR_LO:u32 = 0b00;
const GPIO_OSPEEDR_ME:u32 = 0b01;
const GPIO_OSPEEDR_HI:u32 = 0b10;
const GPIO_OSPEEDR_VH:u32 = 0b11;

const GPIO_PUPDR_NO:u32 = 0b00;
const GPIO_PUPDR_PU:u32 = 0b01;
const GPIO_PUPDR_PD:u32 = 0b10;


const PRESSED: u8 = 1;
const RELEASED: u8 = 0;

#[entry]
fn main() -> ! {
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 0)); //GPIO A
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 3)); //GPIO D
    
    let my_led = ('D', 12);
    let my_but = ('A', 0);


    gpio::gpiod_moder_write(rep_bits(gpio::gpiod_moder_read(), my_led.1*2, 2, GPIO_MODER_OUT));

    gpio::gpioa_moder_write(rep_bits(gpio::gpioa_moder_read(), my_but.1*2, 2, GPIO_MODER_IN));
    gpio::gpioa_pupdr_write(rep_bits(gpio::gpioa_pupdr_read(), my_but.1*2, 2, GPIO_PUPDR_PD));

    //gpio::gpiod_moder_write(1 << (my_led.1*2));
    // GPIOD_OTYPER = GPIOD_OTYPER & ~( 1 << (GREEN_LED));

    let mut bstate = RELEASED;
    gpio::gpiod_bsrr_write(1 << (my_led.1 + 16));
    loop {
        if bstate == RELEASED {
            if (gpio::gpioa_idr_read() & (1 << my_but.1)) != 0 {
                bstate = PRESSED;
                gpio::gpiod_bsrr_write(1 << my_led.1);
            }
        } else {
            if (gpio::gpioa_idr_read() & (1 << my_but.1)) == 0 {
                bstate = RELEASED;
                gpio::gpiod_bsrr_write(1 << (my_led.1 + 16));
            }
        }
        
        
        
    }
}
