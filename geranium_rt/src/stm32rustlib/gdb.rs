extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;


const ITM_ADR : u32 = 0xE0000000;
const DWT_ADR : u32 = 0xE0001000;
const FPB_ADR : u32 = 0xE0002000;
const SCB_ADR : u32 = 0xE000ED00;
const DCB_ADR : u32 = 0xE000EDF0;
        
const ITM_STIMULUS_PORT0_OFFSET : u32 = 0x000;
const ITM_STIMULUS_PORT1_OFFSET : u32 = 0x004;
const ITM_STIMULUS_PORT2_OFFSET : u32 = 0x008;
const ITM_STIMULUS_PORT3_OFFSET : u32 = 0x00C;
const ITM_STIMULUS_PORT4_OFFSET : u32 = 0x010;
const ITM_STIMULUS_PORT5_OFFSET : u32 = 0x014;
const ITM_STIMULUS_PORT6_OFFSET : u32 = 0x018;
const ITM_STIMULUS_PORT7_OFFSET : u32 = 0x01C;
const ITM_STIMULUS_PORT8_OFFSET : u32 = 0x020;
const ITM_STIMULUS_PORT9_OFFSET : u32 = 0x024;
const ITM_STIMULUS_PORT10_OFFSET : u32 = 0x028;
const ITM_STIMULUS_PORT11_OFFSET : u32 = 0x02C;
const ITM_STIMULUS_PORT12_OFFSET : u32 = 0x030;
const ITM_STIMULUS_PORT13_OFFSET : u32 = 0x034;
const ITM_STIMULUS_PORT14_OFFSET : u32 = 0x038;
const ITM_STIMULUS_PORT15_OFFSET : u32 = 0x03C;
const ITM_STIMULUS_PORT16_OFFSET : u32 = 0x040;
const ITM_STIMULUS_PORT17_OFFSET : u32 = 0x044;
const ITM_STIMULUS_PORT18_OFFSET : u32 = 0x048;
const ITM_STIMULUS_PORT19_OFFSET : u32 = 0x04C;
const ITM_STIMULUS_PORT20_OFFSET : u32 = 0x050;
const ITM_STIMULUS_PORT21_OFFSET : u32 = 0x054;
const ITM_STIMULUS_PORT22_OFFSET : u32 = 0x058;
const ITM_STIMULUS_PORT23_OFFSET : u32 = 0x05C;
const ITM_STIMULUS_PORT24_OFFSET : u32 = 0x060;
const ITM_STIMULUS_PORT25_OFFSET : u32 = 0x064;
const ITM_STIMULUS_PORT26_OFFSET : u32 = 0x068;
const ITM_STIMULUS_PORT27_OFFSET : u32 = 0x06C;
const ITM_STIMULUS_PORT28_OFFSET : u32 = 0x070;
const ITM_STIMULUS_PORT29_OFFSET : u32 = 0x074;
const ITM_STIMULUS_PORT30_OFFSET : u32 = 0x078;
const ITM_STIMULUS_PORT31_OFFSET : u32 = 0x07C;
const ITM_TER_OFFSET : u32 = 0xE00;
const ITM_TPR_OFFSET : u32 = 0xE40;
const ITM_TCR_OFFSET : u32 = 0xE80;
const ITM_LAR_OFFSET : u32 = 0xFB0;
const ITM_LSR_OFFSET : u32 = 0xFB4;
    
        
const DWT_CTRL_OFFSET : u32 = 0x00;
const DWT_CYCCNT_OFFSET : u32 = 0x04;
const DWT_CPICNT_OFFSET : u32 = 0x08;
const DWT_EXCCNT_OFFSET : u32 = 0x0C;
const DWT_SLEEPCNT_OFFSET : u32 = 0x10;
const DWT_LSUCNT_OFFSET : u32 = 0x14;
const DWT_FOLDCNT_OFFSET : u32 = 0x18;
const DWT_PCSR_OFFSET : u32 = 0x1C;
const DWT_COMP0_OFFSET : u32 = 0x20;
const DWT_MASK0_OFFSET : u32 = 0x24;
const DWT_FUNCTION0_OFFSET : u32 = 0x28;
const DWT_COMP1_OFFSET : u32 = 0x30;
const DWT_MASK1_OFFSET : u32 = 0x34;
const DWT_FUNCTION1_OFFSET : u32 = 0x38;
const DWT_COMP2_OFFSET : u32 = 0x40;
const DWT_MASK2_OFFSET : u32 = 0x44;
const DWT_FUNCTION2_OFFSET : u32 = 0x48;
const DWT_COMP3_OFFSET : u32 = 0x50;
const DWT_MASK3_OFFSET : u32 = 0x54;
const DWT_FUNCTION3_OFFSET : u32 = 0x58;
const DWT_COMP4_OFFSET : u32 = 0x60;
const DWT_MASK4_OFFSET : u32 = 0x64;
const DWT_FUNCTION4_OFFSET : u32 = 0x68;
const DWT_COMP5_OFFSET : u32 = 0x70;
const DWT_MASK5_OFFSET : u32 = 0x74;
const DWT_FUNCTION5_OFFSET : u32 = 0x78;
const DWT_COMP6_OFFSET : u32 = 0x80;
const DWT_MASK6_OFFSET : u32 = 0x84;
const DWT_FUNCTION6_OFFSET : u32 = 0x88;
const DWT_COMP7_OFFSET : u32 = 0x90;
const DWT_MASK7_OFFSET : u32 = 0x94;
const DWT_FUNCTION7_OFFSET : u32 = 0x98;
const DWT_COMP8_OFFSET : u32 = 0xA0;
const DWT_MASK8_OFFSET : u32 = 0xA4;
const DWT_FUNCTION8_OFFSET : u32 = 0xA8;
const DWT_COMP9_OFFSET : u32 = 0xB0;
const DWT_MASK9_OFFSET : u32 = 0xB4;
const DWT_FUNCTION9_OFFSET : u32 = 0xB8;
const DWT_COMP10_OFFSET : u32 = 0xC0;
const DWT_MASK10_OFFSET : u32 = 0xC4;
const DWT_FUNCTION10_OFFSET : u32 = 0xC8;
const DWT_COMP11_OFFSET : u32 = 0xD0;
const DWT_MASK11_OFFSET : u32 = 0xD4;
const DWT_FUNCTION11_OFFSET : u32 = 0xD8;
const DWT_COMP12_OFFSET : u32 = 0xE0;
const DWT_MASK12_OFFSET : u32 = 0xE4;
const DWT_FUNCTION12_OFFSET : u32 = 0xE8;
const DWT_COMP13_OFFSET : u32 = 0xF0;
const DWT_MASK13_OFFSET : u32 = 0xF4;
const DWT_FUNCTION13_OFFSET : u32 = 0xF8;
const DWT_COMP14_OFFSET : u32 = 0x100;
const DWT_MASK14_OFFSET : u32 = 0x104;
const DWT_FUNCTION14_OFFSET : u32 = 0x108;
const DWT_COMP15_OFFSET : u32 = 0x110;
const DWT_MASK15_OFFSET : u32 = 0x114;
const DWT_FUNCTION15_OFFSET : u32 = 0x118;
const DWT_LSR_OFFSET : u32 = 0xFB4;
    
        
const FPB_CTRL_OFFSET : u32 = 0x00;
const FPB_REMAP_OFFSET : u32 = 0x04;
const FPB_COMP_OFFSET : u32 = 0x08;
const FPB_LSR_OFFSET : u32 = 0xFB4;
    
        
const SCB_CPUID_OFFSET : u32 = 0x00;
    
        
const DCB_DHCSR_OFFSET : u32 = 0x00;
const DCB_DCRSR_OFFSET : u32 = 0x04;
const DCB_DCRDR_OFFSET : u32 = 0x08;
const DCB_DEMCR_OFFSET : u32 = 0x0C;
    
pub const ITM_TRACE_EN_PORT0 : u32 = 1 << 0;
pub const ITM_TRACE_EN_PORT1 : u32 = 1 << 1;
pub const ITM_TRACE_EN_PORT2 : u32 = 1 << 2;
pub const ITM_TRACE_EN_PORT3 : u32 = 1 << 3;
pub const ITM_TRACE_EN_PORT4 : u32 = 1 << 4;
pub const ITM_TRACE_EN_PORT5 : u32 = 1 << 5;
pub const ITM_TRACE_EN_PORT6 : u32 = 1 << 6;
pub const ITM_TRACE_EN_PORT7 : u32 = 1 << 7;
pub const ITM_TRACE_EN_PORT8 : u32 = 1 << 8;
pub const ITM_TRACE_EN_PORT9 : u32 = 1 << 9;
pub const ITM_TRACE_EN_PORT10 : u32 = 1 << 10;
pub const ITM_TRACE_EN_PORT11 : u32 = 1 << 11;
pub const ITM_TRACE_EN_PORT12 : u32 = 1 << 12;
pub const ITM_TRACE_EN_PORT13 : u32 = 1 << 13;
pub const ITM_TRACE_EN_PORT14 : u32 = 1 << 14;
pub const ITM_TRACE_EN_PORT15 : u32 = 1 << 15;
pub const ITM_TRACE_EN_PORT16 : u32 = 1 << 16;
pub const ITM_TRACE_EN_PORT17 : u32 = 1 << 17;
pub const ITM_TRACE_EN_PORT18 : u32 = 1 << 18;
pub const ITM_TRACE_EN_PORT19 : u32 = 1 << 19;
pub const ITM_TRACE_EN_PORT20 : u32 = 1 << 20;
pub const ITM_TRACE_EN_PORT21 : u32 = 1 << 21;
pub const ITM_TRACE_EN_PORT22 : u32 = 1 << 22;
pub const ITM_TRACE_EN_PORT23 : u32 = 1 << 23;
pub const ITM_TRACE_EN_PORT24 : u32 = 1 << 24;
pub const ITM_TRACE_EN_PORT25 : u32 = 1 << 25;
pub const ITM_TRACE_EN_PORT26 : u32 = 1 << 26;
pub const ITM_TRACE_EN_PORT27 : u32 = 1 << 27;
pub const ITM_TRACE_EN_PORT28 : u32 = 1 << 28;
pub const ITM_TRACE_EN_PORT29 : u32 = 1 << 29;
pub const ITM_TRACE_EN_PORT30 : u32 = 1 << 30;
pub const ITM_TRACE_EN_PORT31 : u32 = 1 << 31;
pub const ITM_TRACE_PRIVILEGE_PORT0_7 : u32 = 1 << 0;
pub const ITM_TRACE_PRIVILEGE_PORT8_15 : u32 = 1 << 1;
pub const ITM_TRACE_PRIVILEGE_PORT16_23 : u32 = 1 << 2;
pub const ITM_TRACE_PRIVILEGE_PORT24_31 : u32 = 1 << 3;
pub const ITM_TRACE_CONTROL_ITMENA : u32 = 1 << 0;
pub const ITM_TRACE_CONTROL_TSENA : u32 = 1 << 1;
pub const ITM_TRACE_CONTROL_SYNCENA : u32 = 1 << 2;
pub const ITM_TRACE_CONTROL_DWTENA : u32 = 1 << 3;
pub const ITM_TRACE_CONTROL_SWOENA : u32 = 1 << 4;
pub const ITM_TRACE_CONTROL_TSPRESCALE : u32 = 1 << 8;
pub const ITM_TRACE_CONTROL_ATB_ID : u32 = 1 << 16;
pub const DCB_DEMCR_VC_CORERESET : u32 = 1 << 0;
pub const DCB_DEMCR_VC_HARDERR : u32 = 1 << 4;
pub const DCB_DEMCR_VC_INTERR : u32 = 1 << 5;
pub const DCB_DEMCR_VC_BUSERR : u32 = 1 << 6;
pub const DCB_DEMCR_VC_STATERR : u32 = 1 << 7;
pub const DCB_DEMCR_VC_CHKERR : u32 = 1 << 8;
pub const DCB_DEMCR_VC_NOCPERR : u32 = 1 << 9;
pub const DCB_DEMCR_VC_MMERR : u32 = 1 << 10;
pub const DCB_DEMCR_MON_REQ : u32 = 1 << 16;
pub const DCB_DEMCR_MON_STEP : u32 = 1 << 17;
pub const DCB_DEMCR_MON_PEND : u32 = 1 << 18;
pub const DCB_DEMCR_MON_EN : u32 = 1 << 19;
pub const DCB_DEMCR_TRCENA : u32 = 1 << 24;
        
pub fn itm_stimulus_port0_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT0_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port1_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT1_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port2_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT2_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port3_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT3_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port4_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT4_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port5_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT5_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port6_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT6_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port7_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT7_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port8_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT8_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port9_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT9_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port10_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT10_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port11_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT11_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port12_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT12_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port13_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT13_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port14_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT14_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port15_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT15_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port16_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT16_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port17_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT17_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port18_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT18_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port19_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT19_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port20_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT20_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port21_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT21_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port22_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT22_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port23_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT23_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port24_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT24_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port25_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT25_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port26_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT26_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port27_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT27_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port28_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT28_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port29_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT29_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port30_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT30_OFFSET) as *mut u32, value)
    };
}
pub fn itm_stimulus_port31_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_STIMULUS_PORT31_OFFSET) as *mut u32, value)
    };
}
pub fn itm_ter_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_TER_OFFSET) as *mut u32, value)
    };
}
pub fn itm_tpr_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_TPR_OFFSET) as *mut u32, value)
    };
}
pub fn itm_tcr_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_TCR_OFFSET) as *mut u32, value)
    };
}
pub fn itm_lar_write(value: u32) {
    unsafe {
        write_volatile( (ITM_ADR + ITM_LAR_OFFSET) as *mut u32, value)
    };
}

    
        
pub fn dwt_ctrl_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_CTRL_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_cyccnt_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_CYCCNT_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_cpicnt_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_CPICNT_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_exccnt_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_EXCCNT_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_sleepcnt_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_SLEEPCNT_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_lsucnt_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_LSUCNT_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_foldcnt_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FOLDCNT_OFFSET) as *mut u32, value)
    };
}

pub fn dwt_comp0_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP0_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask0_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK0_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function0_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION0_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp1_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP1_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask1_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK1_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function1_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION1_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp2_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP2_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask2_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK2_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function2_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION2_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp3_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP3_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask3_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK3_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function3_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION3_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp4_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP4_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask4_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK4_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function4_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION4_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp5_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP5_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask5_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK5_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function5_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION5_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp6_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP6_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask6_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK6_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function6_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION6_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp7_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP7_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask7_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK7_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function7_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION7_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp8_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP8_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask8_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK8_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function8_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION8_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp9_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP9_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask9_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK9_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function9_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION9_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp10_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP10_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask10_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK10_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function10_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION10_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp11_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP11_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask11_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK11_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function11_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION11_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp12_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP12_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask12_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK12_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function12_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION12_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp13_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP13_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask13_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK13_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function13_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION13_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp14_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP14_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask14_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK14_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function14_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION14_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_comp15_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_COMP15_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_mask15_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_MASK15_OFFSET) as *mut u32, value)
    };
}
pub fn dwt_function15_write(value: u32) {
    unsafe {
        write_volatile( (DWT_ADR + DWT_FUNCTION15_OFFSET) as *mut u32, value)
    };
}

    
        
pub fn fpb_ctrl_write(value: u32) {
    unsafe {
        write_volatile( (FPB_ADR + FPB_CTRL_OFFSET) as *mut u32, value)
    };
}
pub fn fpb_remap_write(value: u32) {
    unsafe {
        write_volatile( (FPB_ADR + FPB_REMAP_OFFSET) as *mut u32, value)
    };
}
pub fn fpb_comp_write(value: u32) {
    unsafe {
        write_volatile( (FPB_ADR + FPB_COMP_OFFSET) as *mut u32, value)
    };
}

    
        

    
        
pub fn dcb_dhcsr_write(value: u32) {
    unsafe {
        write_volatile( (DCB_ADR + DCB_DHCSR_OFFSET) as *mut u32, value)
    };
}
pub fn dcb_dcrsr_write(value: u32) {
    unsafe {
        write_volatile( (DCB_ADR + DCB_DCRSR_OFFSET) as *mut u32, value)
    };
}
pub fn dcb_dcrdr_write(value: u32) {
    unsafe {
        write_volatile( (DCB_ADR + DCB_DCRDR_OFFSET) as *mut u32, value)
    };
}
pub fn dcb_demcr_write(value: u32) {
    unsafe {
        write_volatile( (DCB_ADR + DCB_DEMCR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn itm_stimulus_port0_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT0_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port1_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT1_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port2_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT2_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port3_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT3_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port4_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT4_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port5_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT5_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port6_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT6_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port7_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT7_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port8_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT8_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port9_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT9_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port10_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT10_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port11_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT11_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port12_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT12_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port13_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT13_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port14_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT14_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port15_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT15_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port16_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT16_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port17_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT17_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port18_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT18_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port19_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT19_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port20_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT20_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port21_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT21_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port22_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT22_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port23_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT23_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port24_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT24_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port25_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT25_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port26_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT26_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port27_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT27_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port28_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT28_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port29_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT29_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port30_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT30_OFFSET) as *mut u32)
    }
}
pub fn itm_stimulus_port31_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_STIMULUS_PORT31_OFFSET) as *mut u32)
    }
}
pub fn itm_ter_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_TER_OFFSET) as *mut u32)
    }
}
pub fn itm_tpr_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_TPR_OFFSET) as *mut u32)
    }
}
pub fn itm_tcr_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_TCR_OFFSET) as *mut u32)
    }
}

pub fn itm_lsr_read() -> u32 {
    unsafe {
        read_volatile( (ITM_ADR + ITM_LSR_OFFSET) as *mut u32)
    }
}
    
        
pub fn dwt_ctrl_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_CTRL_OFFSET) as *mut u32)
    }
}
pub fn dwt_cyccnt_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_CYCCNT_OFFSET) as *mut u32)
    }
}
pub fn dwt_cpicnt_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_CPICNT_OFFSET) as *mut u32)
    }
}
pub fn dwt_exccnt_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_EXCCNT_OFFSET) as *mut u32)
    }
}
pub fn dwt_sleepcnt_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_SLEEPCNT_OFFSET) as *mut u32)
    }
}
pub fn dwt_lsucnt_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_LSUCNT_OFFSET) as *mut u32)
    }
}
pub fn dwt_foldcnt_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FOLDCNT_OFFSET) as *mut u32)
    }
}
pub fn dwt_pcsr_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_PCSR_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp0_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP0_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask0_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK0_OFFSET) as *mut u32)
    }
}
pub fn dwt_function0_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION0_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp1_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP1_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask1_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK1_OFFSET) as *mut u32)
    }
}
pub fn dwt_function1_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION1_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp2_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP2_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask2_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK2_OFFSET) as *mut u32)
    }
}
pub fn dwt_function2_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION2_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp3_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP3_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask3_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK3_OFFSET) as *mut u32)
    }
}
pub fn dwt_function3_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION3_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp4_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP4_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask4_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK4_OFFSET) as *mut u32)
    }
}
pub fn dwt_function4_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION4_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp5_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP5_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask5_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK5_OFFSET) as *mut u32)
    }
}
pub fn dwt_function5_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION5_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp6_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP6_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask6_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK6_OFFSET) as *mut u32)
    }
}
pub fn dwt_function6_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION6_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp7_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP7_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask7_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK7_OFFSET) as *mut u32)
    }
}
pub fn dwt_function7_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION7_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp8_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP8_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask8_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK8_OFFSET) as *mut u32)
    }
}
pub fn dwt_function8_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION8_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp9_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP9_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask9_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK9_OFFSET) as *mut u32)
    }
}
pub fn dwt_function9_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION9_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp10_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP10_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask10_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK10_OFFSET) as *mut u32)
    }
}
pub fn dwt_function10_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION10_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp11_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP11_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask11_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK11_OFFSET) as *mut u32)
    }
}
pub fn dwt_function11_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION11_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp12_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP12_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask12_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK12_OFFSET) as *mut u32)
    }
}
pub fn dwt_function12_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION12_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp13_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP13_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask13_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK13_OFFSET) as *mut u32)
    }
}
pub fn dwt_function13_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION13_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp14_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP14_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask14_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK14_OFFSET) as *mut u32)
    }
}
pub fn dwt_function14_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION14_OFFSET) as *mut u32)
    }
}
pub fn dwt_comp15_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_COMP15_OFFSET) as *mut u32)
    }
}
pub fn dwt_mask15_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_MASK15_OFFSET) as *mut u32)
    }
}
pub fn dwt_function15_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_FUNCTION15_OFFSET) as *mut u32)
    }
}
pub fn dwt_lsr_read() -> u32 {
    unsafe {
        read_volatile( (DWT_ADR + DWT_LSR_OFFSET) as *mut u32)
    }
}
    
        
pub fn fpb_ctrl_read() -> u32 {
    unsafe {
        read_volatile( (FPB_ADR + FPB_CTRL_OFFSET) as *mut u32)
    }
}
pub fn fpb_remap_read() -> u32 {
    unsafe {
        read_volatile( (FPB_ADR + FPB_REMAP_OFFSET) as *mut u32)
    }
}
pub fn fpb_comp_read() -> u32 {
    unsafe {
        read_volatile( (FPB_ADR + FPB_COMP_OFFSET) as *mut u32)
    }
}
pub fn fpb_lsr_read() -> u32 {
    unsafe {
        read_volatile( (FPB_ADR + FPB_LSR_OFFSET) as *mut u32)
    }
}
    
        
pub fn scb_cpuid_read() -> u32 {
    unsafe {
        read_volatile( (SCB_ADR + SCB_CPUID_OFFSET) as *mut u32)
    }
}
    
        
pub fn dcb_dhcsr_read() -> u32 {
    unsafe {
        read_volatile( (DCB_ADR + DCB_DHCSR_OFFSET) as *mut u32)
    }
}

pub fn dcb_dcrdr_read() -> u32 {
    unsafe {
        read_volatile( (DCB_ADR + DCB_DCRDR_OFFSET) as *mut u32)
    }
}
pub fn dcb_demcr_read() -> u32 {
    unsafe {
        read_volatile( (DCB_ADR + DCB_DEMCR_OFFSET) as *mut u32)
    }
}
    
