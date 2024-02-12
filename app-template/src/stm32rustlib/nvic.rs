
extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;
const NVIC_ADR : u32 = 0xE000E000;
        
const NVIC_ISER_OFFSET : u32 = 0x100;
const NVIC_ICER_OFFSET : u32 = 0x180;
const NVIC_ISPR_OFFSET : u32 = 0x200;
const NVIC_ICPR_OFFSET : u32 = 0x280;
const NVIC_IABR_OFFSET : u32 = 0x300;
const NVIC_IPR_OFFSET : u32 = 0x400;
    
pub const NVIC_IRQ : u32 = 0x00000040;
pub const WWDG : u32 = 0;
pub const PVD : u32 = 1;
pub const TAMP_STAMP : u32 = 2;
pub const RTC_WKUP : u32 = 3;
pub const FLASH : u32 = 4;
pub const RCC : u32 = 5;
pub const EXTI0 : u32 = 6;
pub const EXTI1 : u32 = 7;
pub const EXTI2 : u32 = 8;
pub const EXTI3 : u32 = 9;
pub const EXTI4 : u32 = 10;
pub const DMA1_Stream0 : u32 = 11;
pub const DMA1_Stream1 : u32 = 12;
pub const DMA1_Stream2 : u32 = 13;
pub const DMA1_Stream3 : u32 = 14;
pub const DMA1_Stream4 : u32 = 15;
pub const DMA1_Stream5 : u32 = 16;
pub const DMA1_Stream6 : u32 = 17;
pub const ADC : u32 = 18;
pub const CAN1_TX : u32 = 19;
pub const CAN1_RX0 : u32 = 20;
pub const CAN1_RX1 : u32 = 21;
pub const CAN1_SCE : u32 = 22;
pub const EXTI9_5 : u32 = 23;
pub const TIM1_BRK_TIM9 : u32 = 24;
pub const TIM1_UP_TIM10 : u32 = 25;
pub const TIM1_TRG_COM_TIM11 : u32 = 26;
pub const TIM1_CC : u32 = 27;
pub const TIM2 : u32 = 28;
pub const TIM3 : u32 = 29;
pub const TIM4 : u32 = 30;
pub const I2C1_EV : u32 = 31;
pub const I2C1_ER : u32 = 32;
pub const I2C2_EV : u32 = 33;
pub const I2C2_ER : u32 = 34;
pub const SPI1 : u32 = 35;
pub const SPI2 : u32 = 36;
pub const USART1 : u32 = 37;
pub const USART2 : u32 = 38;
pub const USART3 : u32 = 39;
pub const EXTI15_10 : u32 = 40;
pub const RTC_Alarm : u32 = 41;
pub const OTG_FS_WKUP : u32 = 42;
pub const TIM8_BRK_TIM12 : u32 = 43;
pub const TIM8_UP_TIM13 : u32 = 44;
pub const TIM8_TRG_COM_TIM14 : u32 = 45;
pub const TIM8_CC : u32 = 46;
pub const DMA1_Stream7 : u32 = 47;
pub const FSMC : u32 = 48;
pub const SDIO : u32 = 49;
pub const TIM5 : u32 = 50;
pub const SPI3 : u32 = 51;
pub const UART4 : u32 = 52;
pub const UART5 : u32 = 53;
pub const TIM6_DAC : u32 = 54;
pub const TIM7 : u32 = 55;
pub const DMA2_Stream0 : u32 = 56;
pub const DMA2_Stream1 : u32 = 57;
pub const DMA2_Stream2 : u32 = 58;
pub const DMA2_Stream3 : u32 = 59;
pub const DMA2_Stream4 : u32 = 60;
pub const ETH : u32 = 61;
pub const ETH_WKUP : u32 = 62;
pub const CAN2_TX : u32 = 63;
pub const CAN2_RX0 : u32 = 64;
pub const CAN2_RX1 : u32 = 65;
pub const CAN2_SCE : u32 = 66;
pub const OTG_FS : u32 = 67;
pub const DMA2_Stream5 : u32 = 68;
pub const DMA2_Stream6 : u32 = 69;
pub const DMA2_Stream7 : u32 = 70;
pub const USART6 : u32 = 71;
pub const I2C3_EV : u32 = 72;
pub const I2C3_ER : u32 = 73;
pub const OTG_HS_EP1_OUT : u32 = 74;
pub const OTG_HS_EP1_IN : u32 = 75;
pub const OTG_HS_WKUP : u32 = 76;
pub const OTG_HS : u32 = 77;
pub const DCMI : u32 = 78;
pub const CRYP : u32 = 79;
pub const HASH_RNG : u32 = 80;
pub const FPU : u32 = 81;
        






    
        






    

pub fn nvic_iser_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( (NVIC_ADR + NVIC_ISER_OFFSET + (vector)*4 ) as *mut u32, value)
    };
}
pub fn nvic_icer_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( (NVIC_ADR + NVIC_ICER_OFFSET + (vector)*4 ) as *mut u32, value)
    };
}
pub fn nvic_ispr_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( (NVIC_ADR + NVIC_ISPR_OFFSET + (vector)*4 ) as *mut u32, value)
    };
}
pub fn nvic_icpr_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( (NVIC_ADR + NVIC_ICPR_OFFSET + (vector)*4 ) as *mut u32, value)
    };
}
pub fn nvic_iabr_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( (NVIC_ADR + NVIC_IABR_OFFSET + (vector)*4 ) as *mut u32, value)
    };
}
pub fn nvic_ipr_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( (NVIC_ADR + NVIC_IPR_OFFSET + (vector)*4 ) as *mut u32, value)
    };
}pub fn nvic_handler_set(vector: u32, f: unsafe extern "C" fn()){
    unsafe {
        write_volatile( (NVIC_IRQ + vector * 4) as *mut u32, f as u32);
    }
}