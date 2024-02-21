use geranium_seq::sequencer::task::*;
use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::various::HIGH;
use geranium_rt::stm32rustlib::various::LOW;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::delay::*;
use geranium_rt::println;

const MY_LED: (char, u32) = ('D', 12); // Built-in green led

#[derive(Debug)]
pub struct LedOn {
    cbs: u32 // counter before self-destruct
}

impl Task for LedOn {
    fn execute(&mut self) -> () {
        println!("{:?}", self);
        digital_write(MY_LED, HIGH);
        self.cbs += 1;
        if self.cbs == 20 {
            delay_ms(500);
        }
    }

    fn new() -> LedOn {
        LedOn { cbs: 0}
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
        println!("{:?}", self);
        digital_write(MY_LED, LOW);
    }

    fn new() -> LedOff {
        LedOff {}
    }

    fn init(&mut self) {
        println!("init {:?}", self);
    }
}
