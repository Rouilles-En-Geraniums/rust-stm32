
extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;

const GPIOA_ADR : u32 = 0x40020000;
const GPIOB_ADR : u32 = 0x40020400;
const GPIOC_ADR : u32 = 0x40020800;
const GPIOD_ADR : u32 = 0x40020C00;
const GPIOE_ADR : u32 = 0x40021000;
const GPIOF_ADR : u32 = 0x40021400;
const GPIOG_ADR : u32 = 0x40021800;
const GPIOH_ADR : u32 = 0x40021C00;
const GPIOI_ADR : u32 = 0x40022000;
const GPIOJ_ADR : u32 = 0x40022400;
const GPIOK_ADR : u32 = 0x40022800;
        
const GPIOA_MODER_OFFSET : u32 = 0x00;
const GPIOA_OTYPER_OFFSET : u32 = 0x04;
const GPIOA_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOA_PUPDR_OFFSET : u32 = 0x0c;
const GPIOA_IDR_OFFSET : u32 = 0x10;
const GPIOA_ODR_OFFSET : u32 = 0x14;
const GPIOA_BSRR_OFFSET : u32 = 0x18;
const GPIOA_LCKR_OFFSET : u32 = 0x1c;
const GPIOA_AFRL_OFFSET : u32 = 0x20;
const GPIOA_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOB_MODER_OFFSET : u32 = 0x00;
const GPIOB_OTYPER_OFFSET : u32 = 0x04;
const GPIOB_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOB_PUPDR_OFFSET : u32 = 0x0c;
const GPIOB_IDR_OFFSET : u32 = 0x10;
const GPIOB_ODR_OFFSET : u32 = 0x14;
const GPIOB_BSRR_OFFSET : u32 = 0x18;
const GPIOB_LCKR_OFFSET : u32 = 0x1c;
const GPIOB_AFRL_OFFSET : u32 = 0x20;
const GPIOB_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOC_MODER_OFFSET : u32 = 0x00;
const GPIOC_OTYPER_OFFSET : u32 = 0x04;
const GPIOC_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOC_PUPDR_OFFSET : u32 = 0x0c;
const GPIOC_IDR_OFFSET : u32 = 0x10;
const GPIOC_ODR_OFFSET : u32 = 0x14;
const GPIOC_BSRR_OFFSET : u32 = 0x18;
const GPIOC_LCKR_OFFSET : u32 = 0x1c;
const GPIOC_AFRL_OFFSET : u32 = 0x20;
const GPIOC_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOD_MODER_OFFSET : u32 = 0x00;
const GPIOD_OTYPER_OFFSET : u32 = 0x04;
const GPIOD_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOD_PUPDR_OFFSET : u32 = 0x0c;
const GPIOD_IDR_OFFSET : u32 = 0x10;
const GPIOD_ODR_OFFSET : u32 = 0x14;
const GPIOD_BSRR_OFFSET : u32 = 0x18;
const GPIOD_LCKR_OFFSET : u32 = 0x1c;
const GPIOD_AFRL_OFFSET : u32 = 0x20;
const GPIOD_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOE_MODER_OFFSET : u32 = 0x00;
const GPIOE_OTYPER_OFFSET : u32 = 0x04;
const GPIOE_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOE_PUPDR_OFFSET : u32 = 0x0c;
const GPIOE_IDR_OFFSET : u32 = 0x10;
const GPIOE_ODR_OFFSET : u32 = 0x14;
const GPIOE_BSRR_OFFSET : u32 = 0x18;
const GPIOE_LCKR_OFFSET : u32 = 0x1c;
const GPIOE_AFRL_OFFSET : u32 = 0x20;
const GPIOE_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOF_MODER_OFFSET : u32 = 0x00;
const GPIOF_OTYPER_OFFSET : u32 = 0x04;
const GPIOF_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOF_PUPDR_OFFSET : u32 = 0x0c;
const GPIOF_IDR_OFFSET : u32 = 0x10;
const GPIOF_ODR_OFFSET : u32 = 0x14;
const GPIOF_BSRR_OFFSET : u32 = 0x18;
const GPIOF_LCKR_OFFSET : u32 = 0x1c;
const GPIOF_AFRL_OFFSET : u32 = 0x20;
const GPIOF_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOG_MODER_OFFSET : u32 = 0x00;
const GPIOG_OTYPER_OFFSET : u32 = 0x04;
const GPIOG_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOG_PUPDR_OFFSET : u32 = 0x0c;
const GPIOG_IDR_OFFSET : u32 = 0x10;
const GPIOG_ODR_OFFSET : u32 = 0x14;
const GPIOG_BSRR_OFFSET : u32 = 0x18;
const GPIOG_LCKR_OFFSET : u32 = 0x1c;
const GPIOG_AFRL_OFFSET : u32 = 0x20;
const GPIOG_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOH_MODER_OFFSET : u32 = 0x00;
const GPIOH_OTYPER_OFFSET : u32 = 0x04;
const GPIOH_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOH_PUPDR_OFFSET : u32 = 0x0c;
const GPIOH_IDR_OFFSET : u32 = 0x10;
const GPIOH_ODR_OFFSET : u32 = 0x14;
const GPIOH_BSRR_OFFSET : u32 = 0x18;
const GPIOH_LCKR_OFFSET : u32 = 0x1c;
const GPIOH_AFRL_OFFSET : u32 = 0x20;
const GPIOH_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOI_MODER_OFFSET : u32 = 0x00;
const GPIOI_OTYPER_OFFSET : u32 = 0x04;
const GPIOI_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOI_PUPDR_OFFSET : u32 = 0x0c;
const GPIOI_IDR_OFFSET : u32 = 0x10;
const GPIOI_ODR_OFFSET : u32 = 0x14;
const GPIOI_BSRR_OFFSET : u32 = 0x18;
const GPIOI_LCKR_OFFSET : u32 = 0x1c;
const GPIOI_AFRL_OFFSET : u32 = 0x20;
const GPIOI_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOJ_MODER_OFFSET : u32 = 0x00;
const GPIOJ_OTYPER_OFFSET : u32 = 0x04;
const GPIOJ_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOJ_PUPDR_OFFSET : u32 = 0x0c;
const GPIOJ_IDR_OFFSET : u32 = 0x10;
const GPIOJ_ODR_OFFSET : u32 = 0x14;
const GPIOJ_BSRR_OFFSET : u32 = 0x18;
const GPIOJ_LCKR_OFFSET : u32 = 0x1c;
const GPIOJ_AFRL_OFFSET : u32 = 0x20;
const GPIOJ_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOK_MODER_OFFSET : u32 = 0x00;
const GPIOK_OTYPER_OFFSET : u32 = 0x04;
const GPIOK_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOK_PUPDR_OFFSET : u32 = 0x0c;
const GPIOK_IDR_OFFSET : u32 = 0x10;
const GPIOK_ODR_OFFSET : u32 = 0x14;
const GPIOK_BSRR_OFFSET : u32 = 0x18;
const GPIOK_LCKR_OFFSET : u32 = 0x1c;
const GPIOK_AFRL_OFFSET : u32 = 0x20;
const GPIOK_AFRH_OFFSET : u32 = 0x24;
    
pub const GPIO_MODER_IN : u32 = 0b00;
pub const GPIO_MODER_OUT : u32 = 0b01;
pub const GPIO_MODER_ALT : u32 = 0b10;
pub const GPIO_MODER_ANA : u32 = 0b11;
pub const GPIO_OSPEEDR_LO : u32 = 0b00;
pub const GPIO_OSPEEDR_ME : u32 = 0b01;
pub const GPIO_OSPEEDR_HI : u32 = 0b10;
pub const GPIO_OSPEEDR_VH : u32 = 0b11;
pub const GPIO_PUPDR_NO : u32 = 0b00;
pub const GPIO_PUPDR_PU : u32 = 0b01;
pub const GPIO_PUPDR_PD : u32 = 0b10;
        
pub fn gpioa_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiob_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioc_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiod_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioe_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiof_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiog_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioh_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioi_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioj_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiok_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioa_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioa_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioa_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioa_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiob_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiob_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiob_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiob_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioc_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioc_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioc_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioc_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiod_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiod_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiod_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiod_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioe_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioe_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioe_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioe_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiof_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiof_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiof_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiof_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiog_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiog_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiog_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiog_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioh_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioh_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioh_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioh_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioi_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioi_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioi_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioi_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioj_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioj_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioj_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioj_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiok_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiok_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiok_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiok_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_AFRH_OFFSET) as *mut u32)
    }
}
    


/*
fn initGPIO(pin: (char,u8), mode: u8){
    GPIOD_MODER = REP_BITS(GPIOD_MODER, (GREEN_LED)*2, 2, GPIO_MODER_OUT) ;
    GPIOD_OTYPER = GPIOD_OTYPER & ~( 1 << (GREEN_LED));
}
*/

/**
 * pin = (GPIO : char, Pin : u8)
 * mode = HIGH/LOW
 */
pub fn digital_write(pin: (char,u32), mode: u8){
    // pin = (A, 2), mode = 1
    match pin.0 {
        'A' => {
            match mode {
                various::HIGH => gpioa_bsrr_write(1 << pin.1),
                various::LOW => gpioa_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'B' => {
            match mode {
                various::HIGH => gpiob_bsrr_write(1 << pin.1),
                various::LOW => gpiob_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'C' => {
            match mode {
                various::HIGH => gpioc_bsrr_write(1 << pin.1),
                various::LOW => gpioc_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'D' => {
            match mode {
                various::HIGH => gpiod_bsrr_write(1 << pin.1),
                various::LOW => gpiod_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'E' => {
            match mode {
                various::HIGH => gpioe_bsrr_write(1 << pin.1),
                various::LOW => gpioe_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'F' => {
            match mode {
                various::HIGH => gpiof_bsrr_write(1 << pin.1),
                various::LOW => gpiof_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'G' => {
            match mode {
                various::HIGH => gpiog_bsrr_write(1 << pin.1),
                various::LOW => gpiog_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'H' => {
            match mode {
                various::HIGH => gpioh_bsrr_write(1 << pin.1),
                various::LOW => gpioh_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'I' => {
            match mode {
                various::HIGH => gpioi_bsrr_write(1 << pin.1),
                various::LOW => gpioi_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'J' => {
            match mode {
                various::HIGH => gpioj_bsrr_write(1 << pin.1),
                various::LOW => gpioj_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'K' => {
            match mode {
                various::HIGH => gpiok_bsrr_write(1 << pin.1),
                various::LOW => gpiok_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        _ => (),
    }
}

pub fn digital_read(pin: (char,u32)) -> u8 {
    // pin = (A, 2), mode = 1
    match pin.0 {
        'A' => {
            if (gpioa_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'B' => {
            if (gpiob_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'C' => {
            if (gpioc_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'D' => {
            if (gpiod_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'E' => {
            if (gpioe_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'F' => {
            if (gpiof_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'G' => {
            if (gpiog_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'H' => {
            if (gpioh_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'I' => {
            if (gpioi_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'J' => {
            if (gpioj_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        'K' => {
            if (gpiok_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
        _ => 2
    }
}