extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;


const ADC1_ADR : u32 = 0x40012000;
const ADC2_ADR : u32 = 0x40012100;
const ADC3_ADR : u32 = 0x40012200;
        
const ADC1_SR_OFFSET : u32 = 0x00;
const ADC1_CR1_OFFSET : u32 = 0x04;
const ADC1_CR2_OFFSET : u32 = 0x08;
const ADC1_SMPR1_OFFSET : u32 = 0x0C;
const ADC1_SMPR2_OFFSET : u32 = 0x10;
const ADC1_JOFR1_OFFSET : u32 = 0x14;
const ADC1_HTR_OFFSET : u32 = 0x24;
const ADC1_LTR_OFFSET : u32 = 0x28;
const ADC1_SQR1_OFFSET : u32 = 0x2C;
const ADC1_SQR2_OFFSET : u32 = 0x30;
const ADC1_SQR3_OFFSET : u32 = 0x34;
const ADC1_JSQR_OFFSET : u32 = 0x38;
const ADC1_JDR1_OFFSET : u32 = 0x3C;
const ADC1_DR_OFFSET : u32 = 0x4C;
    
        
const ADC2_SR_OFFSET : u32 = 0x00;
const ADC2_CR1_OFFSET : u32 = 0x04;
const ADC2_CR2_OFFSET : u32 = 0x08;
const ADC2_SMPR1_OFFSET : u32 = 0x0C;
const ADC2_SMPR2_OFFSET : u32 = 0x10;
const ADC2_JOFR1_OFFSET : u32 = 0x14;
const ADC2_HTR_OFFSET : u32 = 0x24;
const ADC2_LTR_OFFSET : u32 = 0x28;
const ADC2_SQR1_OFFSET : u32 = 0x2C;
const ADC2_SQR2_OFFSET : u32 = 0x30;
const ADC2_SQR3_OFFSET : u32 = 0x34;
const ADC2_JSQR_OFFSET : u32 = 0x38;
const ADC2_JDR1_OFFSET : u32 = 0x3C;
const ADC2_DR_OFFSET : u32 = 0x4C;
    
        
const ADC3_SR_OFFSET : u32 = 0x00;
const ADC3_CR1_OFFSET : u32 = 0x04;
const ADC3_CR2_OFFSET : u32 = 0x08;
const ADC3_SMPR1_OFFSET : u32 = 0x0C;
const ADC3_SMPR2_OFFSET : u32 = 0x10;
const ADC3_JOFR1_OFFSET : u32 = 0x14;
const ADC3_HTR_OFFSET : u32 = 0x24;
const ADC3_LTR_OFFSET : u32 = 0x28;
const ADC3_SQR1_OFFSET : u32 = 0x2C;
const ADC3_SQR2_OFFSET : u32 = 0x30;
const ADC3_SQR3_OFFSET : u32 = 0x34;
const ADC3_JSQR_OFFSET : u32 = 0x38;
const ADC3_JDR1_OFFSET : u32 = 0x3C;
const ADC3_DR_OFFSET : u32 = 0x4C;
    
pub const ADC_SR_AWD : u32 = 1 << 16;
pub const ADC_SR_EOC : u32 = 1 << 17;
pub const ADC_SR_JEOC : u32 = 1 << 18;
pub const ADC_SR_JSTRT : u32 = 1 << 19;
pub const ADC_SR_STRT : u32 = 1 << 20;
pub const ADC_SR_OVR : u32 = 1 << 21;
pub const ADC_CR1_AWDCH : u32 = 1 << 0;
pub const ADC_CR1_EOCIE : u32 = 1 << 5;
pub const ADC_CR1_AWDIE : u32 = 1 << 6;
pub const ADC_CR1_JEOCIE : u32 = 1 << 7;
pub const ADC_CR1_SCAN : u32 = 1 << 8;
pub const ADC_CR1_AWDSGL : u32 = 1 << 9;
pub const ADC_CR1_JAUTO : u32 = 1 << 10;
pub const ADC_CR1_DISCEN : u32 = 1 << 11;
pub const ADC_CR1_JDISCEN : u32 = 1 << 12;
pub const ADC_CR1_DISCNUM : u32 = 1 << 13;
pub const ADC_CR1_JAWDEN : u32 = 1 << 22;
pub const ADC_CR1_AWDEN : u32 = 1 << 23;
pub const ADC_CR1_OVRIE : u32 = 1 << 26;
pub const ADC_CR2_ADON : u32 = 1 << 0;
pub const ADC_CR2_CONT : u32 = 1 << 1;
pub const ADC_CR2_DMA : u32 = 1 << 8;
pub const ADC_CR2_DDS : u32 = 1 << 9;
pub const ADC_CR2_EOCS : u32 = 1 << 10;
pub const ADC_CR2_ALIGN : u32 = 1 << 11;
pub const ADC_CR2_JEXTSEL : u32 = 1 << 16;
pub const ADC_CR2_JEXTEN : u32 = 1 << 20;
pub const ADC_CR2_JSWSTART : u32 = 1 << 22;
pub const ADC_CR2_EXTSEL : u32 = 1 << 24;
pub const ADC_CR2_EXTSEN : u32 = 1 << 28;
pub const ADC_CR2_SWSTART : u32 = 1 << 30;
pub const ADC_SMPR1_SMP10 : u32 = 1 << 0;
pub const ADC_SMPR1_SMP11 : u32 = 1 << 3;
pub const ADC_SMPR1_SMP12 : u32 = 1 << 6;
pub const ADC_SMPR1_SMP13 : u32 = 1 << 9;
pub const ADC_SMPR1_SMP14 : u32 = 1 << 12;
pub const ADC_SMPR1_SMP15_0 : u32 = 1 << 15;
pub const ADC_SMPR1_SMP15 : u32 = 1 << 16;
pub const ADC_SMPR1_SMP16 : u32 = 1 << 18;
pub const ADC_SMPR1_SMP17 : u32 = 1 << 21;
pub const ADC_SMPR1_SMP18 : u32 = 1 << 24;
pub const ADC_SMPR2_SMP0 : u32 = 1 << 0;
pub const ADC_SMPR2_SMP1 : u32 = 1 << 3;
pub const ADC_SMPR2_SMP2 : u32 = 1 << 6;
pub const ADC_SMPR2_SMP3 : u32 = 1 << 9;
pub const ADC_SMPR2_SMP4 : u32 = 1 << 12;
pub const ADC_SMPR2_SMP5_0 : u32 = 1 << 15;
pub const ADC_SMPR2_SMP5 : u32 = 1 << 16;
pub const ADC_SMPR2_SMP6 : u32 = 1 << 18;
pub const ADC_SMPR2_SMP7 : u32 = 1 << 21;
pub const ADC_SMPR2_SMP8 : u32 = 1 << 24;
pub const ADC_SMPR2_SMP9 : u32 = 1 << 27;
pub const ADC_JOFR1_JOFFSET1 : u32 = 1 << 0;
pub const ADC_HTR_HT : u32 = 1 << 0;
pub const ADC_LTR_LT : u32 = 1 << 0;
pub const ADC_SQR1_SQ13 : u32 = 1 << 0;
pub const ADC_SQR1_SQ14 : u32 = 1 << 5;
pub const ADC_SQR1_SQ15 : u32 = 1 << 10;
pub const ADC_SQR1_SQ15_0 : u32 = 1 << 15;
pub const ADC_SQR1_SQ16 : u32 = 1 << 16;
pub const ADC_SQR1_L : u32 = 1 << 20;
pub const ADC_SQR2_SQ7 : u32 = 1 << 0;
pub const ADC_SQR2_SQ8 : u32 = 1 << 5;
pub const ADC_SQR2_SQ9 : u32 = 1 << 10;
pub const ADC_SQR2_SQ10_0 : u32 = 1 << 15;
pub const ADC_SQR2_SQ10 : u32 = 1 << 16;
pub const ADC_SQR2_SQ11 : u32 = 1 << 20;
pub const ADC_SQR2_SQ12 : u32 = 1 << 25;
pub const ADC_SQR3_SQ1 : u32 = 1 << 0;
pub const ADC_SQR3_SQ2 : u32 = 1 << 5;
pub const ADC_SQR3_SQ3 : u32 = 1 << 10;
pub const ADC_SQR3_SQ4_0 : u32 = 1 << 15;
pub const ADC_SQR3_SQ4 : u32 = 1 << 16;
pub const ADC_SQR3_SQ5 : u32 = 1 << 20;
pub const ADC_SQR3_SQ6 : u32 = 1 << 25;
pub const ADC_JSQR_JSQ1 : u32 = 1 << 0;
pub const ADC_JSQR_JSQ2 : u32 = 1 << 5;
pub const ADC_JSQR_JSQ3 : u32 = 1 << 10;
pub const ADC_JSQR_JSQ4_0 : u32 = 1 << 15;
pub const ADC_JSQR_JSQ4 : u32 = 1 << 16;
pub const ADC_JSQR_JL : u32 = 1 << 20;
pub const ADC_JDR1_JDATA : u32 = 1 << 0;
pub const ADC_DR_DATA : u32 = 1 << 0;
        
pub fn adc1_sr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SR_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_cr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_cr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_smpr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SMPR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_smpr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SMPR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_jofr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_JOFR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_htr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_HTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_ltr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_LTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_sqr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SQR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_sqr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SQR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_sqr3_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SQR3_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_jsqr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_JSQR_OFFSET) as *mut u32, value)
    };
}


    
        
pub fn adc2_sr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SR_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_cr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_cr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_smpr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SMPR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_smpr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SMPR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_jofr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_JOFR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_htr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_HTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_ltr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_LTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_sqr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SQR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_sqr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SQR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_sqr3_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SQR3_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_jsqr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_JSQR_OFFSET) as *mut u32, value)
    };
}


    
        
pub fn adc3_sr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SR_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_cr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_cr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_smpr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SMPR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_smpr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SMPR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_jofr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_JOFR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_htr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_HTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_ltr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_LTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_sqr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SQR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_sqr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SQR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_sqr3_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SQR3_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_jsqr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_JSQR_OFFSET) as *mut u32, value)
    };
}


    
        
pub fn adc1_sr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SR_OFFSET) as *mut u32)
    }
}
pub fn adc1_cr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_CR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_cr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_CR2_OFFSET) as *mut u32)
    }
}
pub fn adc1_smpr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SMPR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_smpr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SMPR2_OFFSET) as *mut u32)
    }
}
pub fn adc1_jofr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_JOFR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_htr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_HTR_OFFSET) as *mut u32)
    }
}
pub fn adc1_ltr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_LTR_OFFSET) as *mut u32)
    }
}
pub fn adc1_sqr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SQR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_sqr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SQR2_OFFSET) as *mut u32)
    }
}
pub fn adc1_sqr3_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SQR3_OFFSET) as *mut u32)
    }
}
pub fn adc1_jsqr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_JSQR_OFFSET) as *mut u32)
    }
}
pub fn adc1_jdr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_JDR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_dr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_DR_OFFSET) as *mut u32)
    }
}
    
        
pub fn adc2_sr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SR_OFFSET) as *mut u32)
    }
}
pub fn adc2_cr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_CR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_cr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_CR2_OFFSET) as *mut u32)
    }
}
pub fn adc2_smpr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SMPR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_smpr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SMPR2_OFFSET) as *mut u32)
    }
}
pub fn adc2_jofr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_JOFR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_htr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_HTR_OFFSET) as *mut u32)
    }
}
pub fn adc2_ltr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_LTR_OFFSET) as *mut u32)
    }
}
pub fn adc2_sqr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SQR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_sqr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SQR2_OFFSET) as *mut u32)
    }
}
pub fn adc2_sqr3_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SQR3_OFFSET) as *mut u32)
    }
}
pub fn adc2_jsqr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_JSQR_OFFSET) as *mut u32)
    }
}
pub fn adc2_jdr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_JDR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_dr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_DR_OFFSET) as *mut u32)
    }
}
    
        
pub fn adc3_sr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SR_OFFSET) as *mut u32)
    }
}
pub fn adc3_cr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_CR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_cr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_CR2_OFFSET) as *mut u32)
    }
}
pub fn adc3_smpr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SMPR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_smpr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SMPR2_OFFSET) as *mut u32)
    }
}
pub fn adc3_jofr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_JOFR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_htr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_HTR_OFFSET) as *mut u32)
    }
}
pub fn adc3_ltr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_LTR_OFFSET) as *mut u32)
    }
}
pub fn adc3_sqr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SQR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_sqr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SQR2_OFFSET) as *mut u32)
    }
}
pub fn adc3_sqr3_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SQR3_OFFSET) as *mut u32)
    }
}
pub fn adc3_jsqr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_JSQR_OFFSET) as *mut u32)
    }
}
pub fn adc3_jdr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_JDR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_dr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_DR_OFFSET) as *mut u32)
    }
}
    
