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
    
