use geranium_seq::sequencer::task::*;
use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::various::HIGH;
use geranium_rt::stm32rustlib::various::LOW;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::delay::*;
use geranium_rt::println;

const MY_LED: (char, u32) = ('D', 12); // Built-in green led

#[derive(Debug)]
pub struct LedOn {}

impl Task for LedOn {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, HIGH);
    }

    fn new() -> LedOn {
        LedOn {}
    }

    fn init(&mut self) {
        println!("init {:?}", self);
        rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
        gpiod_moder_set(MY_LED.1*2, 2, GPIO_MODER_OUT);
    }
}

#[derive(Debug)]
pub struct LedOff {}

impl Task for LedOff {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, LOW);
    }

    fn new() -> LedOff {
        LedOff {}
    }

    fn init(&mut self) {
        println!("init {:?}", self);
    }
}
