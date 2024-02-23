#![no_std]
#![no_main]

extern crate core ; 
extern  crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::delay::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::{print, println};



//#define _GPIONUM(c) ((c) - 'A')
//#define GPIO_QTR8_SENSOR _GPIONUM('D')
//// From left to right
//#define QTR8_PIN0 7 // PD7
const QTR8_PIN0: (char,u32) = ('D', 10);
//#define QTR8_PIN1 5
const QTR8_PIN1: (char,u32) = ('D', 5);
//#define QTR8_PIN2 3
const QTR8_PIN2: (char,u32) = ('D', 7);
//#define QTR8_PIN3 1
const QTR8_PIN3: (char,u32) = ('D', 5);
//#define QTR8_PIN4 0
const QTR8_PIN4: (char,u32) = ('D', 4);
//#define QTR8_PIN5 2
const QTR8_PIN5: (char,u32) = ('D', 6);
//#define QTR8_PIN6 4
const QTR8_PIN6: (char,u32) = ('D', 8);
//#define QTR8_PIN7 6
const QTR8_PIN7: (char,u32) = ('D', 9);

//
//#define QTR8_NUM_PINS 8
const QTR8_NUM_PINS: usize = 8;
//
//#define GPIO_QTR8_LED _GPIONUM('C')
const GPIO_QTR8_LED: (char,u32) = ('C', 15);
//#define QTR8_LED_PIN 15
//
//// From left to right
//static const uint32_t qtr8_pins[] = {QTR8_PIN0, QTR8_PIN1, QTR8_PIN2, QTR8_PIN3, QTR8_PIN4, QTR8_PIN5, QTR8_PIN6, QTR8_PIN7};
const qtr8_pins: [(char,u32); 8] = [QTR8_PIN0, QTR8_PIN1, QTR8_PIN2, QTR8_PIN3, QTR8_PIN4, QTR8_PIN5, QTR8_PIN6, QTR8_PIN7];

//
///**
// *basic active wait timer
// * @param us mico second
// */
//void delay_us(uint16_t us) {
//    TIM3_CNT = 0;
//    while (TIM3_CNT < us) NOP;
//}
//
//void set_qtr8_input() {
//    for (int i = 0; i < QTR8_NUM_PINS; ++i) {
//        GPIO_MODER(GPIO_QTR8_SENSOR) =
//            REP_BITS(GPIO_MODER(GPIO_QTR8_SENSOR), qtr8_pins[i]*2, 2, GPIO_MODER_IN);
//
//        // Pull Up => when line is black, IDR is HIGH
//        GPIO_PUPDR(GPIO_QTR8_SENSOR) =
//            REP_BITS(GPIO_MODER(GPIO_QTR8_SENSOR), qtr8_pins[i]*2, 2, GPIO_PUPDR_PD);
//    }
//}
//
fn set_qtr8_input(){
    for i in 0..=QTR8_NUM_PINS-1 {
        gpiod_moder_set(qtr8_pins[i].1*2,2,GPIO_MODER_IN);
        gpiod_pupdr_set(qtr8_pins[i].1*2,2,GPIO_PUPDR_PD);
    }
}

//void set_qtr8_output() {
//    for (int i = 0; i < QTR8_NUM_PINS; ++i) {
//        GPIO_MODER(GPIO_QTR8_SENSOR) =
//            REP_BITS(GPIO_MODER(GPIO_QTR8_SENSOR), qtr8_pins[i]*2, 2, GPIO_MODER_OUT);
//        GPIO_OTYPER(GPIO_QTR8_SENSOR) &= ~(1 << qtr8_pins[i]);
//    }
//}

//
fn set_qtr8_output(){
    for i in 0..=QTR8_NUM_PINS-1{
        gpiod_moder_set(qtr8_pins[i].1*2,2,GPIO_MODER_OUT);
    }
}
//void set_qtr8_high() {
//    for (int i = 0; i < QTR8_NUM_PINS; ++i) {
//        GPIO_BSRR(GPIO_QTR8_SENSOR) = 1 << qtr8_pins[i];
//    }
//}

fn set_qtr8_high(){
    for i in 0..=QTR8_NUM_PINS-1{
        digital_write(qtr8_pins[i],HIGH);
    }
}
//
//// Decay time grows with darkness (deep) =>
////      * low decay time  = white
////      * high decay time = black
//// When sensor value is 0, capacitor discharged so it has seen some reflection of IR.
//// Measure the time it takes to go from high to low to know if it white or black.
//// (((GPIO_PUPDR_PU (Pull Up) reverse it =>  1=discharged, 0=still charged TODO verify)))
//int qtr8_read_sensor(int pin) {
//    return (GPIO_IDR(GPIO_QTR8_SENSOR) & pin);
//}
//

// Function to read the sensor values for all pins
//fn qtr8_read_sensor(pins: &[(char, u32); QTR8_NUM_PINS]) -> [bool; QTR8_NUM_PINS] {
//    let mut sensor_values = [false; QTR8_NUM_PINS];
//    for i in 0..QTR8_NUM_PINS {
//        sensor_values[i] = digital_read(pins[i]) == HIGH;
//    }
//    sensor_values
//}

fn qtr8_read_sensor(pin : (char, u32))-> bool{
    digital_read(pin) == HIGH
}

//void turn_on_qtr8_led() { GPIO_BSRR(GPIO_QTR8_LED) = 1 << GPIO_QTR8_LED; }
fn turn_on_qtr8_led(){ digital_write(GPIO_QTR8_LED,HIGH);}
//void turn_off_qtr8_led() { GPIO_BSRR(GPIO_QTR8_LED) = 1 << (GPIO_QTR8_LED + 16); }
fn turn_off_qtr8_led(){ digital_write(GPIO_QTR8_LED,LOW);}
//
//void init_qtr8() {
//    // init led
//    GPIO_MODER(GPIO_QTR8_LED) =
//            REP_BITS(GPIO_MODER(GPIO_QTR8_LED), QTR8_LED_PIN, 2, GPIO_MODER_OUT);
//    GPIO_OTYPER(GPIO_QTR8_LED) &= ~(1 << QTR8_LED_PIN);
//    turn_off_qtr8_led();
//
//    // init sensors
//    // set_qtr8_output(); // done in led
//}

fn init_qtr8(){
    //init led
    gpioc_moder_set(GPIO_QTR8_LED.1*2,2,GPIO_MODER_OUT);
    turn_off_qtr8_led();
    // init sensors
    set_qtr8_output(); // done in led

}
//
//// TIM3 used for delay_us fonction
//// with 1Mhz frequency, each tick is 1us
//#define DELAY_US_PSC 42
//#define DELAY_US_ARR 0xffff // we don't use ARR, we want to count to the max we can
const DELAY_US_PSC: u32 = 42;
const DELAY_US_ARR: u32 = 0xffff;
//void init_tim3() {
//    TIM3_CR1 = 0;
//    TIM3_PSC = DELAY_US_PSC;
//    TIM3_ARR = DELAY_US_ARR;
//    TIM3_CR1 = TIM_CEN;
//}
fn init_tim3() {
    tim3_cr1_write(0);
    tim3_psc_write(DELAY_US_PSC);
    tim3_arr_write(DELAY_US_ARR);
    tim3_cr1_write(TIM_CEN);
}
// Function to read the line position based on sensor readings
//fn qtr_read_line(pins: &[(char, u32); QTR8_NUM_PINS], black_line: bool, emitters_on: bool) -> u32 {
//    // Turn on/off the IR emitters based on the input parameter
//    if emitters_on {
//        turn_on_qtr8_led();
//    } else {
//        turn_off_qtr8_led();
//    }
//
//    let mut sensor_values = [false; QTR8_NUM_PINS];
//    for i in 0..QTR8_NUM_PINS {
//        let pin_value = digital_read(pins[i]);
//        sensor_values[i] = if pin_value == HIGH { true } else { false };
//    }
//
//    let mut weighted_sum: u32 = 0;
//    let mut sum: u32 = 0;
//
//    for i in 0..QTR8_NUM_PINS {
//        weighted_sum += (i as u32) * if sensor_values[i] { 1 } else { 0 };
//        sum += if sensor_values[i] { 1 } else { 0 };
//    }
//
//    if sum == 0 {
//        // No line detected, return extreme values
//        if black_line {
//            return 1000 * (QTR8_NUM_PINS as u32 - 1);
//        } else {
//            return 0;
//        }
//    }
//
//    (weighted_sum * 1000) / sum
//}
//
//
//

//
//
//int main() {
//	printf("\nStarting...\n");
//
//	// RCC init
//	RCC_AHB1ENR |= RCC_GPIOAEN;
//
//    RCC_AHB1ENR |= RCC_GPIOCEN;
//	RCC_AHB1ENR |= RCC_GPIODEN;
//    //
//    RCC_APB1ENR |= RCC_TIM3EN;
//
//    init_tim3();
//    init_qtr8();
//    while (42) {
//        /*
//         1. Turn on IR LEDs (optional).
//         2. Set the I/O line to an output and drive it high.
//         3. Allow at least 10 μs for the sensor output to rise.
//         4. Make the I/O line an input (high impedance).
//         5. Measure the time for the voltage to decay by waiting for the I/O line to go low.
//         6. Turn off IR LEDs (optional).
//         */
//        printf("Starting measuring\n");
//        turn_on_qtr8_led();
//        set_qtr8_output();
//        set_qtr8_high();
//        delay_us(12);
//        set_qtr8_input();
//
//        TIM3_CNT = 0;
//        while (qtr8_read_sensor(QTR8_PIN0)) NOP;
//        printf("Took %luus to discharge\n", TIM3_CNT);
//
//        // Threshold (TODO we assume that after 6ms it is black)
//        // delay_us(6000);
//
//        turn_off_qtr8_led();
//
//        // wait 5 sec
//        for (int i = 0; i < 500; i++) {
//            delay_us(10*1000); // 10 ms
//        }
//    }
//
//}
//
//
#[no_mangle]
fn main()  {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOCEN);
    rcc_apb1enr_seti(RCC_APB1ENR_TIM3EN);
    init_tim3();
    init_qtr8();
    delay_init_timers();

    loop {
        turn_on_qtr8_led();
        set_qtr8_output();
        set_qtr8_high();
        delay_us(12);
        set_qtr8_input();


        tim3_cnt_write(0);
         
        //let sensor_values = qtr8_read_sensor(&qtr8_pins);
        //for i in 0..QTR8_NUM_PINS {
        //    println!("Sensor {} value: {}", i, sensor_values[i]);
        //}
        while (qtr8_read_sensor(QTR8_PIN0)) {};
        println!("Took {} us to discharge", tim3_cnt_read());
        //delay_us(6000);

        //let line_position = qtr_read_line(&qtr8_pins, true, true); // Example call with black line and emitters on
        //println!("Line position: {}", line_position);


        turn_off_qtr8_led();
        delay_us(10*1000);
        

        
    }
}