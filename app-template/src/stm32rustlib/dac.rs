extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;

const DAC_ADR : u32 = 0x40007400;
        
const DAC_CR_OFFSET : u32 = 0x00;
const DAC_SWTRIGR_OFFSET : u32 = 0x04;
const DAC_DHR12R1_OFFSET : u32 = 0x08;
const DAC_DHR12L1_OFFSET : u32 = 0x0C;
const DAC_DHR8R1_OFFSET : u32 = 0x10;
const DAC_DHR12R2_OFFSET : u32 = 0x14;
const DAC_DHR12L2_OFFSET : u32 = 0x18;
const DAC_DHR8R2_OFFSET : u32 = 0x1C;
const DAC_DHR12RD_OFFSET : u32 = 0x20;
const DAC_DHR12LD_OFFSET : u32 = 0x24;
const DAC_DHR8RD_OFFSET : u32 = 0x28;
const DAC_DOR1_OFFSET : u32 = 0x2C;
const DAC_DOR2_OFFSET : u32 = 0x30;
const DAC_SR_OFFSET : u32 = 0x34;
    
pub const DAC_CR_EN : u32 = 1 << 0;
pub const DAC_CR_BOFF1 : u32 = 1 << 1;
pub const DAC_CR_TEN1 : u32 = 1 << 2;
pub const DAC_CR_TSEL1 : u32 = 1 << 3;
pub const DAC_CR_WAVE1 : u32 = 1 << 6;
pub const DAC_CR_MAMP1 : u32 = 1 << 8;
pub const DAC_CR_DMAEN1 : u32 = 1 << 12;
pub const DAC_CR_DMAUDRIE1 : u32 = 1 << 13;
pub const DAC_CR_EN2 : u32 = 1 << 16;
pub const DAC_CR_BOFF2 : u32 = 1 << 17;
pub const DAC_CR_TEN2 : u32 = 1 << 18;
pub const DAC_CR_TSEL2 : u32 = 1 << 19;
pub const DAC_CR_WAVE2 : u32 = 1 << 22;
pub const DAC_CR_MAMP2 : u32 = 1 << 24;
pub const DAC_CR_DMAEN2 : u32 = 1 << 28;
pub const DAC_CR_DMAUDRIE2 : u32 = 1 << 29;
pub const DAC_SWTRIGR_SWTRIG1 : u32 = 1 << 0;
pub const DAC_SWTRIGR_SWTRIG2 : u32 = 1 << 1;
pub const DAC_DHR12R1_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR12L1_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR8R1_DACC2DHR : u32 = 1 << 0;
pub const DAC_DHR12R2_DACC2DHR : u32 = 1 << 4;
pub const DAC_DHR12RD_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR12RD_DACC2DHR : u32 = 1 << 16;
pub const DAC_DHR12LD_DACC1DHR : u32 = 1 << 4;
pub const DAC_DHR12LD_DACC2DHR : u32 = 1 << 16;
pub const DAC_DHR8RD_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR8RD_DACC2DHR : u32 = 1 << 8;
pub const DAC_DOR1_DACC1DOR : u32 = 1 << 0;
pub const DAC_DOR2_DACC2DOR : u32 = 1 << 0;
pub const DAC_SR_DMAUDR1 : u32 = 1 << 13;
pub const DAC_SR_DMAUDR2 : u32 = 1 << 29;
        
pub fn dac_cr_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_CR_OFFSET) as *mut u32, value)
    };
}
pub fn dac_swtrigr_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_SWTRIGR_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12r1_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12R1_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12l1_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12L1_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr8r1_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR8R1_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12r2_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12R2_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12l2_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12L2_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr8r2_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR8R2_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12rd_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12RD_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12ld_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12LD_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr8rd_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR8RD_OFFSET) as *mut u32, value)
    };
}



    
        
pub fn dac_cr_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_CR_OFFSET) as *mut u32)
    }
}
pub fn dac_swtrigr_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_SWTRIGR_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12r1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12R1_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12l1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12L1_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr8r1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR8R1_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12r2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12R2_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12l2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12L2_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr8r2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR8R2_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12rd_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12RD_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12ld_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12LD_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr8rd_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR8RD_OFFSET) as *mut u32)
    }
}
pub fn dac_dor1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DOR1_OFFSET) as *mut u32)
    }
}
pub fn dac_dor2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DOR2_OFFSET) as *mut u32)
    }
}
pub fn dac_sr_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_SR_OFFSET) as *mut u32)
    }
}
    
