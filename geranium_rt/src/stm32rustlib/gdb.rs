/**
 *	Rust on STM32 Project by Rouilles en GeraniumTM
 *	Copyright (C) 2024 Université de Toulouse :
 *   - Oussama Felfel - oussama.felfel@univ-tlse3.fr
 *   - François Foltete - francois.foltete@univ-tlse3.fr
 *   - Elana Courtines - elana.courtines@univ-tlse3.fr
 *   - Teo Tinarrage - teo.tinarrage@univ-tlse3.fr
 *   - Zineb Moubarik - zineb.moubarik@univ-tlse3.fr
 *
 *  This library aims to provide the following :
 *   - a rust library generation tool to safely access memory ;
 *   - a support to flash STM32 boards ;
 *   - a task scheduling tool that generates the associated rust code.
 *
 *  The development of this library has done as a Proof of Concept and
 *  is currently only tested for STM32F407-G DISC1 Boards.
 *
 *  It is our hope that using this library to enable development on
 *  other boards will be facilitated.
 *
 *
 *	This program is free software: you can redistribute it and/or modify
 *	it under the terms of the GNU General Public License as published by
 *	the Free Software Foundation, either version 3 of the License, or
 *	(at your option) any later version.
 *
 *	This program is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *	GNU General Public License for more details.
**/

extern crate core;

use crate::core::ptr::read_volatile;
use crate::core::ptr::write_volatile;
use crate::stm32rustlib::various::*;

const ITM_ADR: u32 = 0xE0000000;
const DWT_ADR: u32 = 0xE0001000;
const FPB_ADR: u32 = 0xE0002000;
const SCB_ADR: u32 = 0xE000ED00;
const DCB_ADR: u32 = 0xE000EDF0;
        
const ITM_STIMULUS_PORT0_OFFSET: u32 = 0x000;
const ITM_STIMULUS_PORT1_OFFSET: u32 = 0x004;
const ITM_STIMULUS_PORT2_OFFSET: u32 = 0x008;
const ITM_STIMULUS_PORT3_OFFSET: u32 = 0x00C;
const ITM_STIMULUS_PORT4_OFFSET: u32 = 0x010;
const ITM_STIMULUS_PORT5_OFFSET: u32 = 0x014;
const ITM_STIMULUS_PORT6_OFFSET: u32 = 0x018;
const ITM_STIMULUS_PORT7_OFFSET: u32 = 0x01C;
const ITM_STIMULUS_PORT8_OFFSET: u32 = 0x020;
const ITM_STIMULUS_PORT9_OFFSET: u32 = 0x024;
const ITM_STIMULUS_PORT10_OFFSET: u32 = 0x028;
const ITM_STIMULUS_PORT11_OFFSET: u32 = 0x02C;
const ITM_STIMULUS_PORT12_OFFSET: u32 = 0x030;
const ITM_STIMULUS_PORT13_OFFSET: u32 = 0x034;
const ITM_STIMULUS_PORT14_OFFSET: u32 = 0x038;
const ITM_STIMULUS_PORT15_OFFSET: u32 = 0x03C;
const ITM_STIMULUS_PORT16_OFFSET: u32 = 0x040;
const ITM_STIMULUS_PORT17_OFFSET: u32 = 0x044;
const ITM_STIMULUS_PORT18_OFFSET: u32 = 0x048;
const ITM_STIMULUS_PORT19_OFFSET: u32 = 0x04C;
const ITM_STIMULUS_PORT20_OFFSET: u32 = 0x050;
const ITM_STIMULUS_PORT21_OFFSET: u32 = 0x054;
const ITM_STIMULUS_PORT22_OFFSET: u32 = 0x058;
const ITM_STIMULUS_PORT23_OFFSET: u32 = 0x05C;
const ITM_STIMULUS_PORT24_OFFSET: u32 = 0x060;
const ITM_STIMULUS_PORT25_OFFSET: u32 = 0x064;
const ITM_STIMULUS_PORT26_OFFSET: u32 = 0x068;
const ITM_STIMULUS_PORT27_OFFSET: u32 = 0x06C;
const ITM_STIMULUS_PORT28_OFFSET: u32 = 0x070;
const ITM_STIMULUS_PORT29_OFFSET: u32 = 0x074;
const ITM_STIMULUS_PORT30_OFFSET: u32 = 0x078;
const ITM_STIMULUS_PORT31_OFFSET: u32 = 0x07C;
const ITM_TER_OFFSET: u32 = 0xE00;
const ITM_TPR_OFFSET: u32 = 0xE40;
const ITM_TCR_OFFSET: u32 = 0xE80;
const ITM_LAR_OFFSET: u32 = 0xFB0;
const ITM_LSR_OFFSET: u32 = 0xFB4;
    
        
const DWT_CTRL_OFFSET: u32 = 0x00;
const DWT_CYCCNT_OFFSET: u32 = 0x04;
const DWT_CPICNT_OFFSET: u32 = 0x08;
const DWT_EXCCNT_OFFSET: u32 = 0x0C;
const DWT_SLEEPCNT_OFFSET: u32 = 0x10;
const DWT_LSUCNT_OFFSET: u32 = 0x14;
const DWT_FOLDCNT_OFFSET: u32 = 0x18;
const DWT_PCSR_OFFSET: u32 = 0x1C;
const DWT_COMP0_OFFSET: u32 = 0x20;
const DWT_MASK0_OFFSET: u32 = 0x24;
const DWT_FUNCTION0_OFFSET: u32 = 0x28;
const DWT_COMP1_OFFSET: u32 = 0x30;
const DWT_MASK1_OFFSET: u32 = 0x34;
const DWT_FUNCTION1_OFFSET: u32 = 0x38;
const DWT_COMP2_OFFSET: u32 = 0x40;
const DWT_MASK2_OFFSET: u32 = 0x44;
const DWT_FUNCTION2_OFFSET: u32 = 0x48;
const DWT_COMP3_OFFSET: u32 = 0x50;
const DWT_MASK3_OFFSET: u32 = 0x54;
const DWT_FUNCTION3_OFFSET: u32 = 0x58;
const DWT_COMP4_OFFSET: u32 = 0x60;
const DWT_MASK4_OFFSET: u32 = 0x64;
const DWT_FUNCTION4_OFFSET: u32 = 0x68;
const DWT_COMP5_OFFSET: u32 = 0x70;
const DWT_MASK5_OFFSET: u32 = 0x74;
const DWT_FUNCTION5_OFFSET: u32 = 0x78;
const DWT_COMP6_OFFSET: u32 = 0x80;
const DWT_MASK6_OFFSET: u32 = 0x84;
const DWT_FUNCTION6_OFFSET: u32 = 0x88;
const DWT_COMP7_OFFSET: u32 = 0x90;
const DWT_MASK7_OFFSET: u32 = 0x94;
const DWT_FUNCTION7_OFFSET: u32 = 0x98;
const DWT_COMP8_OFFSET: u32 = 0xA0;
const DWT_MASK8_OFFSET: u32 = 0xA4;
const DWT_FUNCTION8_OFFSET: u32 = 0xA8;
const DWT_COMP9_OFFSET: u32 = 0xB0;
const DWT_MASK9_OFFSET: u32 = 0xB4;
const DWT_FUNCTION9_OFFSET: u32 = 0xB8;
const DWT_COMP10_OFFSET: u32 = 0xC0;
const DWT_MASK10_OFFSET: u32 = 0xC4;
const DWT_FUNCTION10_OFFSET: u32 = 0xC8;
const DWT_COMP11_OFFSET: u32 = 0xD0;
const DWT_MASK11_OFFSET: u32 = 0xD4;
const DWT_FUNCTION11_OFFSET: u32 = 0xD8;
const DWT_COMP12_OFFSET: u32 = 0xE0;
const DWT_MASK12_OFFSET: u32 = 0xE4;
const DWT_FUNCTION12_OFFSET: u32 = 0xE8;
const DWT_COMP13_OFFSET: u32 = 0xF0;
const DWT_MASK13_OFFSET: u32 = 0xF4;
const DWT_FUNCTION13_OFFSET: u32 = 0xF8;
const DWT_COMP14_OFFSET: u32 = 0x100;
const DWT_MASK14_OFFSET: u32 = 0x104;
const DWT_FUNCTION14_OFFSET: u32 = 0x108;
const DWT_COMP15_OFFSET: u32 = 0x110;
const DWT_MASK15_OFFSET: u32 = 0x114;
const DWT_FUNCTION15_OFFSET: u32 = 0x118;
const DWT_LSR_OFFSET: u32 = 0xFB4;
    
        
const FPB_CTRL_OFFSET: u32 = 0x00;
const FPB_REMAP_OFFSET: u32 = 0x04;
const FPB_COMP_OFFSET: u32 = 0x08;
const FPB_LSR_OFFSET: u32 = 0xFB4;
    
        
const SCB_CPUID_OFFSET: u32 = 0x00;
    
        
const DCB_DHCSR_OFFSET: u32 = 0x00;
const DCB_DCRSR_OFFSET: u32 = 0x04;
const DCB_DCRDR_OFFSET: u32 = 0x08;
const DCB_DEMCR_OFFSET: u32 = 0x0C;
    
pub const ITM_TRACE_EN_PORT0: u32 = 1 << 0;
pub const ITM_TRACE_EN_PORT1: u32 = 1 << 1;
pub const ITM_TRACE_EN_PORT2: u32 = 1 << 2;
pub const ITM_TRACE_EN_PORT3: u32 = 1 << 3;
pub const ITM_TRACE_EN_PORT4: u32 = 1 << 4;
pub const ITM_TRACE_EN_PORT5: u32 = 1 << 5;
pub const ITM_TRACE_EN_PORT6: u32 = 1 << 6;
pub const ITM_TRACE_EN_PORT7: u32 = 1 << 7;
pub const ITM_TRACE_EN_PORT8: u32 = 1 << 8;
pub const ITM_TRACE_EN_PORT9: u32 = 1 << 9;
pub const ITM_TRACE_EN_PORT10: u32 = 1 << 10;
pub const ITM_TRACE_EN_PORT11: u32 = 1 << 11;
pub const ITM_TRACE_EN_PORT12: u32 = 1 << 12;
pub const ITM_TRACE_EN_PORT13: u32 = 1 << 13;
pub const ITM_TRACE_EN_PORT14: u32 = 1 << 14;
pub const ITM_TRACE_EN_PORT15: u32 = 1 << 15;
pub const ITM_TRACE_EN_PORT16: u32 = 1 << 16;
pub const ITM_TRACE_EN_PORT17: u32 = 1 << 17;
pub const ITM_TRACE_EN_PORT18: u32 = 1 << 18;
pub const ITM_TRACE_EN_PORT19: u32 = 1 << 19;
pub const ITM_TRACE_EN_PORT20: u32 = 1 << 20;
pub const ITM_TRACE_EN_PORT21: u32 = 1 << 21;
pub const ITM_TRACE_EN_PORT22: u32 = 1 << 22;
pub const ITM_TRACE_EN_PORT23: u32 = 1 << 23;
pub const ITM_TRACE_EN_PORT24: u32 = 1 << 24;
pub const ITM_TRACE_EN_PORT25: u32 = 1 << 25;
pub const ITM_TRACE_EN_PORT26: u32 = 1 << 26;
pub const ITM_TRACE_EN_PORT27: u32 = 1 << 27;
pub const ITM_TRACE_EN_PORT28: u32 = 1 << 28;
pub const ITM_TRACE_EN_PORT29: u32 = 1 << 29;
pub const ITM_TRACE_EN_PORT30: u32 = 1 << 30;
pub const ITM_TRACE_EN_PORT31: u32 = 1 << 31;
pub const ITM_TRACE_PRIVILEGE_PORT0_7: u32 = 1 << 0;
pub const ITM_TRACE_PRIVILEGE_PORT8_15: u32 = 1 << 1;
pub const ITM_TRACE_PRIVILEGE_PORT16_23: u32 = 1 << 2;
pub const ITM_TRACE_PRIVILEGE_PORT24_31: u32 = 1 << 3;
pub const ITM_TRACE_CONTROL_ITMENA: u32 = 1 << 0;
pub const ITM_TRACE_CONTROL_TSENA: u32 = 1 << 1;
pub const ITM_TRACE_CONTROL_SYNCENA: u32 = 1 << 2;
pub const ITM_TRACE_CONTROL_DWTENA: u32 = 1 << 3;
pub const ITM_TRACE_CONTROL_SWOENA: u32 = 1 << 4;
pub const ITM_TRACE_CONTROL_TSPRESCALE: u32 = 1 << 8;
pub const ITM_TRACE_CONTROL_ATB_ID: u32 = 1 << 16;
pub const DCB_DEMCR_VC_CORERESET: u32 = 1 << 0;
pub const DCB_DEMCR_VC_HARDERR: u32 = 1 << 4;
pub const DCB_DEMCR_VC_INTERR: u32 = 1 << 5;
pub const DCB_DEMCR_VC_BUSERR: u32 = 1 << 6;
pub const DCB_DEMCR_VC_STATERR: u32 = 1 << 7;
pub const DCB_DEMCR_VC_CHKERR: u32 = 1 << 8;
pub const DCB_DEMCR_VC_NOCPERR: u32 = 1 << 9;
pub const DCB_DEMCR_VC_MMERR: u32 = 1 << 10;
pub const DCB_DEMCR_MON_REQ: u32 = 1 << 16;
pub const DCB_DEMCR_MON_STEP: u32 = 1 << 17;
pub const DCB_DEMCR_MON_PEND: u32 = 1 << 18;
pub const DCB_DEMCR_MON_EN: u32 = 1 << 19;
pub const DCB_DEMCR_TRCENA: u32 = 1 << 24;
        
#[inline(always)]
pub fn itm_stimulus_port0_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT0_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port1_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT1_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port2_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT2_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port3_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT3_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port4_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT4_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port5_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT5_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port6_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT6_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port7_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT7_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port8_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT8_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port9_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT9_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port10_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT10_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port11_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT11_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port12_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT12_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port13_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT13_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port14_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT14_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port15_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT15_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port16_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT16_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port17_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT17_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port18_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT18_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port19_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT19_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port20_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT20_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port21_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT21_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port22_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT22_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port23_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT23_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port24_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT24_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port25_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT25_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port26_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT26_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port27_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT27_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port28_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT28_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port29_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT29_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port30_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT30_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_stimulus_port31_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_STIMULUS_PORT31_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_ter_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_TER_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_tpr_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_TPR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_tcr_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_TCR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn itm_lar_write(value: u32) {
    unsafe { write_volatile((ITM_ADR + ITM_LAR_OFFSET) as *mut u32, value) };
}

    
        
#[inline(always)]
pub fn dwt_ctrl_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_CTRL_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_cyccnt_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_CYCCNT_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_cpicnt_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_CPICNT_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_exccnt_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_EXCCNT_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_sleepcnt_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_SLEEPCNT_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_lsucnt_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_LSUCNT_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_foldcnt_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FOLDCNT_OFFSET) as *mut u32, value) };
}

#[inline(always)]
pub fn dwt_comp0_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP0_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask0_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK0_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function0_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION0_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp1_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP1_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask1_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK1_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function1_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION1_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp2_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP2_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask2_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK2_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function2_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION2_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp3_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP3_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask3_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK3_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function3_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION3_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp4_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP4_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask4_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK4_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function4_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION4_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp5_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP5_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask5_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK5_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function5_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION5_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp6_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP6_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask6_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK6_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function6_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION6_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp7_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP7_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask7_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK7_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function7_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION7_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp8_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP8_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask8_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK8_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function8_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION8_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp9_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP9_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask9_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK9_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function9_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION9_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp10_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP10_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask10_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK10_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function10_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION10_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp11_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP11_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask11_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK11_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function11_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION11_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp12_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP12_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask12_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK12_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function12_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION12_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp13_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP13_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask13_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK13_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function13_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION13_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp14_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP14_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask14_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK14_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function14_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION14_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_comp15_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_COMP15_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_mask15_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_MASK15_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dwt_function15_write(value: u32) {
    unsafe { write_volatile((DWT_ADR + DWT_FUNCTION15_OFFSET) as *mut u32, value) };
}

    
        
#[inline(always)]
pub fn fpb_ctrl_write(value: u32) {
    unsafe { write_volatile((FPB_ADR + FPB_CTRL_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn fpb_remap_write(value: u32) {
    unsafe { write_volatile((FPB_ADR + FPB_REMAP_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn fpb_comp_write(value: u32) {
    unsafe { write_volatile((FPB_ADR + FPB_COMP_OFFSET) as *mut u32, value) };
}

    
        

    
        
#[inline(always)]
pub fn dcb_dhcsr_write(value: u32) {
    unsafe { write_volatile((DCB_ADR + DCB_DHCSR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dcb_dcrsr_write(value: u32) {
    unsafe { write_volatile((DCB_ADR + DCB_DCRSR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dcb_dcrdr_write(value: u32) {
    unsafe { write_volatile((DCB_ADR + DCB_DCRDR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn dcb_demcr_write(value: u32) {
    unsafe { write_volatile((DCB_ADR + DCB_DEMCR_OFFSET) as *mut u32, value) };
}
    
        
#[inline(always)]
pub fn itm_stimulus_port0_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT0_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port1_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT1_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port2_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT2_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port3_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT3_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port4_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT4_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port5_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT5_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port6_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT6_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port7_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT7_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port8_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT8_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port9_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT9_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port10_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT10_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port11_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT11_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port12_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT12_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port13_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT13_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port14_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT14_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port15_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT15_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port16_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT16_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port17_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT17_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port18_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT18_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port19_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT19_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port20_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT20_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port21_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT21_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port22_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT22_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port23_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT23_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port24_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT24_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port25_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT25_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port26_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT26_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port27_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT27_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port28_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT28_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port29_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT29_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port30_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT30_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_stimulus_port31_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_STIMULUS_PORT31_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_ter_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_TER_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_tpr_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_TPR_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn itm_tcr_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_TCR_OFFSET) as *mut u32) }
}

#[inline(always)]
pub fn itm_lsr_read() -> u32 {
    unsafe { read_volatile((ITM_ADR + ITM_LSR_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn dwt_ctrl_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_CTRL_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_cyccnt_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_CYCCNT_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_cpicnt_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_CPICNT_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_exccnt_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_EXCCNT_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_sleepcnt_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_SLEEPCNT_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_lsucnt_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_LSUCNT_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_foldcnt_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FOLDCNT_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_pcsr_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_PCSR_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp0_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP0_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask0_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK0_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function0_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION0_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp1_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP1_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask1_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK1_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function1_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION1_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp2_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP2_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask2_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK2_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function2_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION2_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp3_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP3_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask3_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK3_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function3_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION3_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp4_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP4_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask4_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK4_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function4_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION4_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp5_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP5_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask5_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK5_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function5_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION5_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp6_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP6_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask6_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK6_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function6_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION6_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp7_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP7_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask7_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK7_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function7_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION7_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp8_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP8_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask8_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK8_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function8_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION8_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp9_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP9_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask9_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK9_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function9_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION9_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp10_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP10_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask10_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK10_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function10_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION10_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp11_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP11_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask11_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK11_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function11_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION11_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp12_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP12_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask12_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK12_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function12_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION12_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp13_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP13_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask13_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK13_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function13_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION13_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp14_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP14_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask14_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK14_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function14_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION14_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_comp15_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_COMP15_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_mask15_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_MASK15_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_function15_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_FUNCTION15_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dwt_lsr_read() -> u32 {
    unsafe { read_volatile((DWT_ADR + DWT_LSR_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn fpb_ctrl_read() -> u32 {
    unsafe { read_volatile((FPB_ADR + FPB_CTRL_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn fpb_remap_read() -> u32 {
    unsafe { read_volatile((FPB_ADR + FPB_REMAP_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn fpb_comp_read() -> u32 {
    unsafe { read_volatile((FPB_ADR + FPB_COMP_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn fpb_lsr_read() -> u32 {
    unsafe { read_volatile((FPB_ADR + FPB_LSR_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn scb_cpuid_read() -> u32 {
    unsafe { read_volatile((SCB_ADR + SCB_CPUID_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn dcb_dhcsr_read() -> u32 {
    unsafe { read_volatile((DCB_ADR + DCB_DHCSR_OFFSET) as *mut u32) }
}

#[inline(always)]
pub fn dcb_dcrdr_read() -> u32 {
    unsafe { read_volatile((DCB_ADR + DCB_DCRDR_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn dcb_demcr_read() -> u32 {
    unsafe { read_volatile((DCB_ADR + DCB_DEMCR_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn itm_stimulus_port0_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port0_write(rep_bits(itm_stimulus_port0_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port1_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port1_write(rep_bits(itm_stimulus_port1_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port2_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port2_write(rep_bits(itm_stimulus_port2_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port3_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port3_write(rep_bits(itm_stimulus_port3_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port4_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port4_write(rep_bits(itm_stimulus_port4_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port5_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port5_write(rep_bits(itm_stimulus_port5_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port6_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port6_write(rep_bits(itm_stimulus_port6_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port7_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port7_write(rep_bits(itm_stimulus_port7_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port8_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port8_write(rep_bits(itm_stimulus_port8_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port9_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port9_write(rep_bits(itm_stimulus_port9_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port10_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port10_write(rep_bits(itm_stimulus_port10_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port11_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port11_write(rep_bits(itm_stimulus_port11_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port12_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port12_write(rep_bits(itm_stimulus_port12_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port13_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port13_write(rep_bits(itm_stimulus_port13_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port14_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port14_write(rep_bits(itm_stimulus_port14_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port15_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port15_write(rep_bits(itm_stimulus_port15_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port16_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port16_write(rep_bits(itm_stimulus_port16_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port17_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port17_write(rep_bits(itm_stimulus_port17_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port18_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port18_write(rep_bits(itm_stimulus_port18_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port19_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port19_write(rep_bits(itm_stimulus_port19_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port20_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port20_write(rep_bits(itm_stimulus_port20_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port21_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port21_write(rep_bits(itm_stimulus_port21_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port22_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port22_write(rep_bits(itm_stimulus_port22_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port23_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port23_write(rep_bits(itm_stimulus_port23_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port24_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port24_write(rep_bits(itm_stimulus_port24_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port25_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port25_write(rep_bits(itm_stimulus_port25_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port26_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port26_write(rep_bits(itm_stimulus_port26_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port27_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port27_write(rep_bits(itm_stimulus_port27_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port28_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port28_write(rep_bits(itm_stimulus_port28_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port29_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port29_write(rep_bits(itm_stimulus_port29_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port30_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port30_write(rep_bits(itm_stimulus_port30_read(), position, size, value));
}
#[inline(always)]
pub fn itm_stimulus_port31_set(position: u32, size: u32, value: u32) {
    itm_stimulus_port31_write(rep_bits(itm_stimulus_port31_read(), position, size, value));
}
#[inline(always)]
pub fn itm_ter_set(position: u32, size: u32, value: u32) {
    itm_ter_write(rep_bits(itm_ter_read(), position, size, value));
}
#[inline(always)]
pub fn itm_tpr_set(position: u32, size: u32, value: u32) {
    itm_tpr_write(rep_bits(itm_tpr_read(), position, size, value));
}
#[inline(always)]
pub fn itm_tcr_set(position: u32, size: u32, value: u32) {
    itm_tcr_write(rep_bits(itm_tcr_read(), position, size, value));
}


    
        
#[inline(always)]
pub fn dwt_ctrl_set(position: u32, size: u32, value: u32) {
    dwt_ctrl_write(rep_bits(dwt_ctrl_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_cyccnt_set(position: u32, size: u32, value: u32) {
    dwt_cyccnt_write(rep_bits(dwt_cyccnt_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_cpicnt_set(position: u32, size: u32, value: u32) {
    dwt_cpicnt_write(rep_bits(dwt_cpicnt_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_exccnt_set(position: u32, size: u32, value: u32) {
    dwt_exccnt_write(rep_bits(dwt_exccnt_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_sleepcnt_set(position: u32, size: u32, value: u32) {
    dwt_sleepcnt_write(rep_bits(dwt_sleepcnt_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_lsucnt_set(position: u32, size: u32, value: u32) {
    dwt_lsucnt_write(rep_bits(dwt_lsucnt_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_foldcnt_set(position: u32, size: u32, value: u32) {
    dwt_foldcnt_write(rep_bits(dwt_foldcnt_read(), position, size, value));
}

#[inline(always)]
pub fn dwt_comp0_set(position: u32, size: u32, value: u32) {
    dwt_comp0_write(rep_bits(dwt_comp0_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask0_set(position: u32, size: u32, value: u32) {
    dwt_mask0_write(rep_bits(dwt_mask0_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function0_set(position: u32, size: u32, value: u32) {
    dwt_function0_write(rep_bits(dwt_function0_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp1_set(position: u32, size: u32, value: u32) {
    dwt_comp1_write(rep_bits(dwt_comp1_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask1_set(position: u32, size: u32, value: u32) {
    dwt_mask1_write(rep_bits(dwt_mask1_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function1_set(position: u32, size: u32, value: u32) {
    dwt_function1_write(rep_bits(dwt_function1_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp2_set(position: u32, size: u32, value: u32) {
    dwt_comp2_write(rep_bits(dwt_comp2_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask2_set(position: u32, size: u32, value: u32) {
    dwt_mask2_write(rep_bits(dwt_mask2_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function2_set(position: u32, size: u32, value: u32) {
    dwt_function2_write(rep_bits(dwt_function2_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp3_set(position: u32, size: u32, value: u32) {
    dwt_comp3_write(rep_bits(dwt_comp3_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask3_set(position: u32, size: u32, value: u32) {
    dwt_mask3_write(rep_bits(dwt_mask3_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function3_set(position: u32, size: u32, value: u32) {
    dwt_function3_write(rep_bits(dwt_function3_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp4_set(position: u32, size: u32, value: u32) {
    dwt_comp4_write(rep_bits(dwt_comp4_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask4_set(position: u32, size: u32, value: u32) {
    dwt_mask4_write(rep_bits(dwt_mask4_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function4_set(position: u32, size: u32, value: u32) {
    dwt_function4_write(rep_bits(dwt_function4_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp5_set(position: u32, size: u32, value: u32) {
    dwt_comp5_write(rep_bits(dwt_comp5_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask5_set(position: u32, size: u32, value: u32) {
    dwt_mask5_write(rep_bits(dwt_mask5_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function5_set(position: u32, size: u32, value: u32) {
    dwt_function5_write(rep_bits(dwt_function5_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp6_set(position: u32, size: u32, value: u32) {
    dwt_comp6_write(rep_bits(dwt_comp6_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask6_set(position: u32, size: u32, value: u32) {
    dwt_mask6_write(rep_bits(dwt_mask6_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function6_set(position: u32, size: u32, value: u32) {
    dwt_function6_write(rep_bits(dwt_function6_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp7_set(position: u32, size: u32, value: u32) {
    dwt_comp7_write(rep_bits(dwt_comp7_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask7_set(position: u32, size: u32, value: u32) {
    dwt_mask7_write(rep_bits(dwt_mask7_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function7_set(position: u32, size: u32, value: u32) {
    dwt_function7_write(rep_bits(dwt_function7_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp8_set(position: u32, size: u32, value: u32) {
    dwt_comp8_write(rep_bits(dwt_comp8_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask8_set(position: u32, size: u32, value: u32) {
    dwt_mask8_write(rep_bits(dwt_mask8_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function8_set(position: u32, size: u32, value: u32) {
    dwt_function8_write(rep_bits(dwt_function8_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp9_set(position: u32, size: u32, value: u32) {
    dwt_comp9_write(rep_bits(dwt_comp9_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask9_set(position: u32, size: u32, value: u32) {
    dwt_mask9_write(rep_bits(dwt_mask9_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function9_set(position: u32, size: u32, value: u32) {
    dwt_function9_write(rep_bits(dwt_function9_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp10_set(position: u32, size: u32, value: u32) {
    dwt_comp10_write(rep_bits(dwt_comp10_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask10_set(position: u32, size: u32, value: u32) {
    dwt_mask10_write(rep_bits(dwt_mask10_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function10_set(position: u32, size: u32, value: u32) {
    dwt_function10_write(rep_bits(dwt_function10_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp11_set(position: u32, size: u32, value: u32) {
    dwt_comp11_write(rep_bits(dwt_comp11_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask11_set(position: u32, size: u32, value: u32) {
    dwt_mask11_write(rep_bits(dwt_mask11_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function11_set(position: u32, size: u32, value: u32) {
    dwt_function11_write(rep_bits(dwt_function11_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp12_set(position: u32, size: u32, value: u32) {
    dwt_comp12_write(rep_bits(dwt_comp12_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask12_set(position: u32, size: u32, value: u32) {
    dwt_mask12_write(rep_bits(dwt_mask12_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function12_set(position: u32, size: u32, value: u32) {
    dwt_function12_write(rep_bits(dwt_function12_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp13_set(position: u32, size: u32, value: u32) {
    dwt_comp13_write(rep_bits(dwt_comp13_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask13_set(position: u32, size: u32, value: u32) {
    dwt_mask13_write(rep_bits(dwt_mask13_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function13_set(position: u32, size: u32, value: u32) {
    dwt_function13_write(rep_bits(dwt_function13_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp14_set(position: u32, size: u32, value: u32) {
    dwt_comp14_write(rep_bits(dwt_comp14_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask14_set(position: u32, size: u32, value: u32) {
    dwt_mask14_write(rep_bits(dwt_mask14_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function14_set(position: u32, size: u32, value: u32) {
    dwt_function14_write(rep_bits(dwt_function14_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_comp15_set(position: u32, size: u32, value: u32) {
    dwt_comp15_write(rep_bits(dwt_comp15_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_mask15_set(position: u32, size: u32, value: u32) {
    dwt_mask15_write(rep_bits(dwt_mask15_read(), position, size, value));
}
#[inline(always)]
pub fn dwt_function15_set(position: u32, size: u32, value: u32) {
    dwt_function15_write(rep_bits(dwt_function15_read(), position, size, value));
}

    
        
#[inline(always)]
pub fn fpb_ctrl_set(position: u32, size: u32, value: u32) {
    fpb_ctrl_write(rep_bits(fpb_ctrl_read(), position, size, value));
}
#[inline(always)]
pub fn fpb_remap_set(position: u32, size: u32, value: u32) {
    fpb_remap_write(rep_bits(fpb_remap_read(), position, size, value));
}
#[inline(always)]
pub fn fpb_comp_set(position: u32, size: u32, value: u32) {
    fpb_comp_write(rep_bits(fpb_comp_read(), position, size, value));
}

    
        

    
        
#[inline(always)]
pub fn dcb_dhcsr_set(position: u32, size: u32, value: u32) {
    dcb_dhcsr_write(rep_bits(dcb_dhcsr_read(), position, size, value));
}

#[inline(always)]
pub fn dcb_dcrdr_set(position: u32, size: u32, value: u32) {
    dcb_dcrdr_write(rep_bits(dcb_dcrdr_read(), position, size, value));
}
#[inline(always)]
pub fn dcb_demcr_set(position: u32, size: u32, value: u32) {
    dcb_demcr_write(rep_bits(dcb_demcr_read(), position, size, value));
}
    
        
#[inline(always)]
pub fn itm_stimulus_port0_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port0_write(itm_stimulus_port0_read() | value),
        31 => itm_stimulus_port0_write(itm_stimulus_port0_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port1_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port1_write(itm_stimulus_port1_read() | value),
        31 => itm_stimulus_port1_write(itm_stimulus_port1_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port2_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port2_write(itm_stimulus_port2_read() | value),
        31 => itm_stimulus_port2_write(itm_stimulus_port2_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port3_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port3_write(itm_stimulus_port3_read() | value),
        31 => itm_stimulus_port3_write(itm_stimulus_port3_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port4_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port4_write(itm_stimulus_port4_read() | value),
        31 => itm_stimulus_port4_write(itm_stimulus_port4_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port5_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port5_write(itm_stimulus_port5_read() | value),
        31 => itm_stimulus_port5_write(itm_stimulus_port5_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port6_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port6_write(itm_stimulus_port6_read() | value),
        31 => itm_stimulus_port6_write(itm_stimulus_port6_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port7_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port7_write(itm_stimulus_port7_read() | value),
        31 => itm_stimulus_port7_write(itm_stimulus_port7_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port8_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port8_write(itm_stimulus_port8_read() | value),
        31 => itm_stimulus_port8_write(itm_stimulus_port8_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port9_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port9_write(itm_stimulus_port9_read() | value),
        31 => itm_stimulus_port9_write(itm_stimulus_port9_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port10_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port10_write(itm_stimulus_port10_read() | value),
        31 => itm_stimulus_port10_write(itm_stimulus_port10_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port11_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port11_write(itm_stimulus_port11_read() | value),
        31 => itm_stimulus_port11_write(itm_stimulus_port11_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port12_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port12_write(itm_stimulus_port12_read() | value),
        31 => itm_stimulus_port12_write(itm_stimulus_port12_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port13_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port13_write(itm_stimulus_port13_read() | value),
        31 => itm_stimulus_port13_write(itm_stimulus_port13_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port14_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port14_write(itm_stimulus_port14_read() | value),
        31 => itm_stimulus_port14_write(itm_stimulus_port14_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port15_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port15_write(itm_stimulus_port15_read() | value),
        31 => itm_stimulus_port15_write(itm_stimulus_port15_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port16_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port16_write(itm_stimulus_port16_read() | value),
        31 => itm_stimulus_port16_write(itm_stimulus_port16_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port17_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port17_write(itm_stimulus_port17_read() | value),
        31 => itm_stimulus_port17_write(itm_stimulus_port17_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port18_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port18_write(itm_stimulus_port18_read() | value),
        31 => itm_stimulus_port18_write(itm_stimulus_port18_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port19_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port19_write(itm_stimulus_port19_read() | value),
        31 => itm_stimulus_port19_write(itm_stimulus_port19_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port20_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port20_write(itm_stimulus_port20_read() | value),
        31 => itm_stimulus_port20_write(itm_stimulus_port20_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port21_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port21_write(itm_stimulus_port21_read() | value),
        31 => itm_stimulus_port21_write(itm_stimulus_port21_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port22_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port22_write(itm_stimulus_port22_read() | value),
        31 => itm_stimulus_port22_write(itm_stimulus_port22_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port23_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port23_write(itm_stimulus_port23_read() | value),
        31 => itm_stimulus_port23_write(itm_stimulus_port23_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port24_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port24_write(itm_stimulus_port24_read() | value),
        31 => itm_stimulus_port24_write(itm_stimulus_port24_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port25_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port25_write(itm_stimulus_port25_read() | value),
        31 => itm_stimulus_port25_write(itm_stimulus_port25_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port26_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port26_write(itm_stimulus_port26_read() | value),
        31 => itm_stimulus_port26_write(itm_stimulus_port26_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port27_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port27_write(itm_stimulus_port27_read() | value),
        31 => itm_stimulus_port27_write(itm_stimulus_port27_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port28_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port28_write(itm_stimulus_port28_read() | value),
        31 => itm_stimulus_port28_write(itm_stimulus_port28_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port29_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port29_write(itm_stimulus_port29_read() | value),
        31 => itm_stimulus_port29_write(itm_stimulus_port29_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port30_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port30_write(itm_stimulus_port30_read() | value),
        31 => itm_stimulus_port30_write(itm_stimulus_port30_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_stimulus_port31_seti(value: u32) {
    match value.count_ones() {
        1 => itm_stimulus_port31_write(itm_stimulus_port31_read() | value),
        31 => itm_stimulus_port31_write(itm_stimulus_port31_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_ter_seti(value: u32) {
    match value.count_ones() {
        1 => itm_ter_write(itm_ter_read() | value),
        31 => itm_ter_write(itm_ter_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_tpr_seti(value: u32) {
    match value.count_ones() {
        1 => itm_tpr_write(itm_tpr_read() | value),
        31 => itm_tpr_write(itm_tpr_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn itm_tcr_seti(value: u32) {
    match value.count_ones() {
        1 => itm_tcr_write(itm_tcr_read() | value),
        31 => itm_tcr_write(itm_tcr_read() & value),
        _ => (),
    }
}


    
        
#[inline(always)]
pub fn dwt_ctrl_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_ctrl_write(dwt_ctrl_read() | value),
        31 => dwt_ctrl_write(dwt_ctrl_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_cyccnt_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_cyccnt_write(dwt_cyccnt_read() | value),
        31 => dwt_cyccnt_write(dwt_cyccnt_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_cpicnt_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_cpicnt_write(dwt_cpicnt_read() | value),
        31 => dwt_cpicnt_write(dwt_cpicnt_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_exccnt_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_exccnt_write(dwt_exccnt_read() | value),
        31 => dwt_exccnt_write(dwt_exccnt_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_sleepcnt_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_sleepcnt_write(dwt_sleepcnt_read() | value),
        31 => dwt_sleepcnt_write(dwt_sleepcnt_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_lsucnt_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_lsucnt_write(dwt_lsucnt_read() | value),
        31 => dwt_lsucnt_write(dwt_lsucnt_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_foldcnt_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_foldcnt_write(dwt_foldcnt_read() | value),
        31 => dwt_foldcnt_write(dwt_foldcnt_read() & value),
        _ => (),
    }
}

#[inline(always)]
pub fn dwt_comp0_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp0_write(dwt_comp0_read() | value),
        31 => dwt_comp0_write(dwt_comp0_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask0_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask0_write(dwt_mask0_read() | value),
        31 => dwt_mask0_write(dwt_mask0_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function0_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function0_write(dwt_function0_read() | value),
        31 => dwt_function0_write(dwt_function0_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp1_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp1_write(dwt_comp1_read() | value),
        31 => dwt_comp1_write(dwt_comp1_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask1_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask1_write(dwt_mask1_read() | value),
        31 => dwt_mask1_write(dwt_mask1_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function1_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function1_write(dwt_function1_read() | value),
        31 => dwt_function1_write(dwt_function1_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp2_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp2_write(dwt_comp2_read() | value),
        31 => dwt_comp2_write(dwt_comp2_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask2_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask2_write(dwt_mask2_read() | value),
        31 => dwt_mask2_write(dwt_mask2_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function2_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function2_write(dwt_function2_read() | value),
        31 => dwt_function2_write(dwt_function2_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp3_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp3_write(dwt_comp3_read() | value),
        31 => dwt_comp3_write(dwt_comp3_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask3_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask3_write(dwt_mask3_read() | value),
        31 => dwt_mask3_write(dwt_mask3_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function3_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function3_write(dwt_function3_read() | value),
        31 => dwt_function3_write(dwt_function3_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp4_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp4_write(dwt_comp4_read() | value),
        31 => dwt_comp4_write(dwt_comp4_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask4_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask4_write(dwt_mask4_read() | value),
        31 => dwt_mask4_write(dwt_mask4_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function4_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function4_write(dwt_function4_read() | value),
        31 => dwt_function4_write(dwt_function4_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp5_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp5_write(dwt_comp5_read() | value),
        31 => dwt_comp5_write(dwt_comp5_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask5_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask5_write(dwt_mask5_read() | value),
        31 => dwt_mask5_write(dwt_mask5_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function5_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function5_write(dwt_function5_read() | value),
        31 => dwt_function5_write(dwt_function5_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp6_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp6_write(dwt_comp6_read() | value),
        31 => dwt_comp6_write(dwt_comp6_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask6_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask6_write(dwt_mask6_read() | value),
        31 => dwt_mask6_write(dwt_mask6_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function6_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function6_write(dwt_function6_read() | value),
        31 => dwt_function6_write(dwt_function6_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp7_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp7_write(dwt_comp7_read() | value),
        31 => dwt_comp7_write(dwt_comp7_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask7_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask7_write(dwt_mask7_read() | value),
        31 => dwt_mask7_write(dwt_mask7_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function7_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function7_write(dwt_function7_read() | value),
        31 => dwt_function7_write(dwt_function7_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp8_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp8_write(dwt_comp8_read() | value),
        31 => dwt_comp8_write(dwt_comp8_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask8_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask8_write(dwt_mask8_read() | value),
        31 => dwt_mask8_write(dwt_mask8_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function8_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function8_write(dwt_function8_read() | value),
        31 => dwt_function8_write(dwt_function8_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp9_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp9_write(dwt_comp9_read() | value),
        31 => dwt_comp9_write(dwt_comp9_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask9_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask9_write(dwt_mask9_read() | value),
        31 => dwt_mask9_write(dwt_mask9_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function9_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function9_write(dwt_function9_read() | value),
        31 => dwt_function9_write(dwt_function9_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp10_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp10_write(dwt_comp10_read() | value),
        31 => dwt_comp10_write(dwt_comp10_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask10_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask10_write(dwt_mask10_read() | value),
        31 => dwt_mask10_write(dwt_mask10_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function10_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function10_write(dwt_function10_read() | value),
        31 => dwt_function10_write(dwt_function10_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp11_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp11_write(dwt_comp11_read() | value),
        31 => dwt_comp11_write(dwt_comp11_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask11_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask11_write(dwt_mask11_read() | value),
        31 => dwt_mask11_write(dwt_mask11_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function11_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function11_write(dwt_function11_read() | value),
        31 => dwt_function11_write(dwt_function11_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp12_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp12_write(dwt_comp12_read() | value),
        31 => dwt_comp12_write(dwt_comp12_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask12_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask12_write(dwt_mask12_read() | value),
        31 => dwt_mask12_write(dwt_mask12_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function12_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function12_write(dwt_function12_read() | value),
        31 => dwt_function12_write(dwt_function12_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp13_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp13_write(dwt_comp13_read() | value),
        31 => dwt_comp13_write(dwt_comp13_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask13_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask13_write(dwt_mask13_read() | value),
        31 => dwt_mask13_write(dwt_mask13_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function13_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function13_write(dwt_function13_read() | value),
        31 => dwt_function13_write(dwt_function13_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp14_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp14_write(dwt_comp14_read() | value),
        31 => dwt_comp14_write(dwt_comp14_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask14_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask14_write(dwt_mask14_read() | value),
        31 => dwt_mask14_write(dwt_mask14_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function14_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function14_write(dwt_function14_read() | value),
        31 => dwt_function14_write(dwt_function14_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_comp15_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_comp15_write(dwt_comp15_read() | value),
        31 => dwt_comp15_write(dwt_comp15_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_mask15_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_mask15_write(dwt_mask15_read() | value),
        31 => dwt_mask15_write(dwt_mask15_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dwt_function15_seti(value: u32) {
    match value.count_ones() {
        1 => dwt_function15_write(dwt_function15_read() | value),
        31 => dwt_function15_write(dwt_function15_read() & value),
        _ => (),
    }
}

    
        
#[inline(always)]
pub fn fpb_ctrl_seti(value: u32) {
    match value.count_ones() {
        1 => fpb_ctrl_write(fpb_ctrl_read() | value),
        31 => fpb_ctrl_write(fpb_ctrl_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn fpb_remap_seti(value: u32) {
    match value.count_ones() {
        1 => fpb_remap_write(fpb_remap_read() | value),
        31 => fpb_remap_write(fpb_remap_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn fpb_comp_seti(value: u32) {
    match value.count_ones() {
        1 => fpb_comp_write(fpb_comp_read() | value),
        31 => fpb_comp_write(fpb_comp_read() & value),
        _ => (),
    }
}

    
        

    
        
#[inline(always)]
pub fn dcb_dhcsr_seti(value: u32) {
    match value.count_ones() {
        1 => dcb_dhcsr_write(dcb_dhcsr_read() | value),
        31 => dcb_dhcsr_write(dcb_dhcsr_read() & value),
        _ => (),
    }
}

#[inline(always)]
pub fn dcb_dcrdr_seti(value: u32) {
    match value.count_ones() {
        1 => dcb_dcrdr_write(dcb_dcrdr_read() | value),
        31 => dcb_dcrdr_write(dcb_dcrdr_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn dcb_demcr_seti(value: u32) {
    match value.count_ones() {
        1 => dcb_demcr_write(dcb_demcr_read() | value),
        31 => dcb_demcr_write(dcb_demcr_read() & value),
        _ => (),
    }
}
    