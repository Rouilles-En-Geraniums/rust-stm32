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
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various::*;

const TIM1_ADR : u32 = 0x40010000;
const TIM8_ADR : u32 = 0x40010400;
const TIM2_ADR : u32 = 0x40000000;
const TIM3_ADR : u32 = 0x40000400;
const TIM4_ADR : u32 = 0x40000800;
const TIM5_ADR : u32 = 0x40000C00;
const TIM9_ADR : u32 = 0x40014000;
const TIM12_ADR : u32 = 0x40001800;
const TIM10_ADR : u32 = 0x40014400;
const TIM11_ADR : u32 = 0x40014800;
const TIM13_ADR : u32 = 0x40001C00;
const TIM14_ADR : u32 = 0x40002000;
const TIM6_ADR : u32 = 0x40001000;
const TIM7_ADR : u32 = 0x40001400;
        
const TIM1_CR1_OFFSET : u32 = 0x00;
const TIM1_CR2_OFFSET : u32 = 0x04;
const TIM1_SMCR_OFFSET : u32 = 0x08;
const TIM1_DIER_OFFSET : u32 = 0x0C;
const TIM1_SR_OFFSET : u32 = 0x10;
const TIM1_EGR_OFFSET : u32 = 0x14;
const TIM1_CCMR1_OFFSET : u32 = 0x18;
const TIM1_CCMR2_OFFSET : u32 = 0x1C;
const TIM1_CCER_OFFSET : u32 = 0x20;
const TIM1_CNT_OFFSET : u32 = 0x24;
const TIM1_PSC_OFFSET : u32 = 0x28;
const TIM1_ARR_OFFSET : u32 = 0x2C;
const TIM1_CCR1_OFFSET : u32 = 0x34;
const TIM1_CCR2_OFFSET : u32 = 0x38;
const TIM1_CCR3_OFFSET : u32 = 0x3C;
const TIM1_CCR4_OFFSET : u32 = 0x40;
const TIM1_BTDR_OFFSET : u32 = 0x44;
const TIM1_DCR_OFFSET : u32 = 0x48;
const TIM1_DMAR_OFFSET : u32 = 0x4C;
    
        
const TIM8_CR1_OFFSET : u32 = 0x00;
const TIM8_CR2_OFFSET : u32 = 0x04;
const TIM8_SMCR_OFFSET : u32 = 0x08;
const TIM8_DIER_OFFSET : u32 = 0x0C;
const TIM8_SR_OFFSET : u32 = 0x10;
const TIM8_EGR_OFFSET : u32 = 0x14;
const TIM8_CCMR1_OFFSET : u32 = 0x18;
const TIM8_CCMR2_OFFSET : u32 = 0x1C;
const TIM8_CCER_OFFSET : u32 = 0x20;
const TIM8_CNT_OFFSET : u32 = 0x24;
const TIM8_PSC_OFFSET : u32 = 0x28;
const TIM8_ARR_OFFSET : u32 = 0x2C;
const TIM8_CCR1_OFFSET : u32 = 0x34;
const TIM8_CCR2_OFFSET : u32 = 0x38;
const TIM8_CCR3_OFFSET : u32 = 0x3C;
const TIM8_CCR4_OFFSET : u32 = 0x40;
const TIM8_BTDR_OFFSET : u32 = 0x44;
const TIM8_DCR_OFFSET : u32 = 0x48;
const TIM8_DMAR_OFFSET : u32 = 0x4C;
    
        
const TIM2_CR1_OFFSET : u32 = 0x00;
const TIM2_CR2_OFFSET : u32 = 0x04;
const TIM2_SMCR_OFFSET : u32 = 0x08;
const TIM2_DIER_OFFSET : u32 = 0x0C;
const TIM2_SR_OFFSET : u32 = 0x10;
const TIM2_EGR_OFFSET : u32 = 0x14;
const TIM2_CCMR1_OFFSET : u32 = 0x18;
const TIM2_CCMR2_OFFSET : u32 = 0x1C;
const TIM2_CCER_OFFSET : u32 = 0x20;
const TIM2_CNT_OFFSET : u32 = 0x24;
const TIM2_PSC_OFFSET : u32 = 0x28;
const TIM2_ARR_OFFSET : u32 = 0x2C;
const TIM2_CCR1_OFFSET : u32 = 0x34;
const TIM2_CCR2_OFFSET : u32 = 0x38;
const TIM2_CCR3_OFFSET : u32 = 0x3C;
const TIM2_CCR4_OFFSET : u32 = 0x40;
const TIM2_DCR_OFFSET : u32 = 0x48;
const TIM2_DMAR_OFFSET : u32 = 0x4C;
const TIM2_OR_OFFSET : u32 = 0x50;
    
        
const TIM3_CR1_OFFSET : u32 = 0x00;
const TIM3_CR2_OFFSET : u32 = 0x04;
const TIM3_SMCR_OFFSET : u32 = 0x08;
const TIM3_DIER_OFFSET : u32 = 0x0C;
const TIM3_SR_OFFSET : u32 = 0x10;
const TIM3_EGR_OFFSET : u32 = 0x14;
const TIM3_CCMR1_OFFSET : u32 = 0x18;
const TIM3_CCMR2_OFFSET : u32 = 0x1C;
const TIM3_CCER_OFFSET : u32 = 0x20;
const TIM3_CNT_OFFSET : u32 = 0x24;
const TIM3_PSC_OFFSET : u32 = 0x28;
const TIM3_ARR_OFFSET : u32 = 0x2C;
const TIM3_CCR1_OFFSET : u32 = 0x34;
const TIM3_CCR2_OFFSET : u32 = 0x38;
const TIM3_CCR3_OFFSET : u32 = 0x3C;
const TIM3_CCR4_OFFSET : u32 = 0x40;
const TIM3_DCR_OFFSET : u32 = 0x48;
const TIM3_DMAR_OFFSET : u32 = 0x4C;
    
        
const TIM4_CR1_OFFSET : u32 = 0x00;
const TIM4_CR2_OFFSET : u32 = 0x04;
const TIM4_SMCR_OFFSET : u32 = 0x08;
const TIM4_DIER_OFFSET : u32 = 0x0C;
const TIM4_SR_OFFSET : u32 = 0x10;
const TIM4_EGR_OFFSET : u32 = 0x14;
const TIM4_CCMR1_OFFSET : u32 = 0x18;
const TIM4_CCMR2_OFFSET : u32 = 0x1C;
const TIM4_CCER_OFFSET : u32 = 0x20;
const TIM4_CNT_OFFSET : u32 = 0x24;
const TIM4_PSC_OFFSET : u32 = 0x28;
const TIM4_ARR_OFFSET : u32 = 0x2C;
const TIM4_CCR1_OFFSET : u32 = 0x34;
const TIM4_CCR2_OFFSET : u32 = 0x38;
const TIM4_CCR3_OFFSET : u32 = 0x3C;
const TIM4_CCR4_OFFSET : u32 = 0x40;
const TIM4_DCR_OFFSET : u32 = 0x48;
const TIM4_DMAR_OFFSET : u32 = 0x4C;
    
        
const TIM5_CR1_OFFSET : u32 = 0x00;
const TIM5_CR2_OFFSET : u32 = 0x04;
const TIM5_SMCR_OFFSET : u32 = 0x08;
const TIM5_DIER_OFFSET : u32 = 0x0C;
const TIM5_SR_OFFSET : u32 = 0x10;
const TIM5_EGR_OFFSET : u32 = 0x14;
const TIM5_CCMR1_OFFSET : u32 = 0x18;
const TIM5_CCMR2_OFFSET : u32 = 0x1C;
const TIM5_CCER_OFFSET : u32 = 0x20;
const TIM5_CNT_OFFSET : u32 = 0x24;
const TIM5_PSC_OFFSET : u32 = 0x28;
const TIM5_ARR_OFFSET : u32 = 0x2C;
const TIM5_CCR1_OFFSET : u32 = 0x34;
const TIM5_CCR2_OFFSET : u32 = 0x38;
const TIM5_CCR3_OFFSET : u32 = 0x3C;
const TIM5_CCR4_OFFSET : u32 = 0x40;
const TIM5_DCR_OFFSET : u32 = 0x48;
const TIM5_DMAR_OFFSET : u32 = 0x4C;
const TIM5_OR_OFFSET : u32 = 0x50;
    
        
const TIM9_CR1_OFFSET : u32 = 0x00;
const TIM9_SMCR_OFFSET : u32 = 0x08;
const TIM9_DIER_OFFSET : u32 = 0x0C;
const TIM9_SR_OFFSET : u32 = 0x10;
const TIM9_EGR_OFFSET : u32 = 0x14;
const TIM9_CCMR1_OFFSET : u32 = 0x18;
const TIM9_CCER_OFFSET : u32 = 0x20;
const TIM9_CNT_OFFSET : u32 = 0x24;
const TIM9_PSC_OFFSET : u32 = 0x28;
const TIM9_ARR_OFFSET : u32 = 0x2C;
const TIM9_CCR1_OFFSET : u32 = 0x34;
const TIM9_CCR2_OFFSET : u32 = 0x38;
    
        
const TIM12_CR1_OFFSET : u32 = 0x00;
const TIM12_SMCR_OFFSET : u32 = 0x08;
const TIM12_DIER_OFFSET : u32 = 0x0C;
const TIM12_SR_OFFSET : u32 = 0x10;
const TIM12_EGR_OFFSET : u32 = 0x14;
const TIM12_CCMR1_OFFSET : u32 = 0x18;
const TIM12_CCER_OFFSET : u32 = 0x20;
const TIM12_CNT_OFFSET : u32 = 0x24;
const TIM12_PSC_OFFSET : u32 = 0x28;
const TIM12_ARR_OFFSET : u32 = 0x2C;
const TIM12_CCR1_OFFSET : u32 = 0x34;
const TIM12_CCR2_OFFSET : u32 = 0x38;
    
        
const TIM10_CR1_OFFSET : u32 = 0x00;
const TIM10_DIER_OFFSET : u32 = 0x0C;
const TIM10_SR_OFFSET : u32 = 0x10;
const TIM10_EGR_OFFSET : u32 = 0x14;
const TIM10_CCMR1_OFFSET : u32 = 0x18;
const TIM10_CCER_OFFSET : u32 = 0x20;
const TIM10_CNT_OFFSET : u32 = 0x24;
const TIM10_PSC_OFFSET : u32 = 0x28;
const TIM10_ARR_OFFSET : u32 = 0x2C;
const TIM10_CCR1_OFFSET : u32 = 0x34;
const TIM10_OR_OFFSET : u32 = 0x50;
    
        
const TIM11_CR1_OFFSET : u32 = 0x00;
const TIM11_DIER_OFFSET : u32 = 0x0C;
const TIM11_SR_OFFSET : u32 = 0x10;
const TIM11_EGR_OFFSET : u32 = 0x14;
const TIM11_CCMR1_OFFSET : u32 = 0x18;
const TIM11_CCER_OFFSET : u32 = 0x20;
const TIM11_CNT_OFFSET : u32 = 0x24;
const TIM11_PSC_OFFSET : u32 = 0x28;
const TIM11_ARR_OFFSET : u32 = 0x2C;
const TIM11_CCR1_OFFSET : u32 = 0x34;
const TIM11_OR_OFFSET : u32 = 0x50;
    
        
const TIM13_CR1_OFFSET : u32 = 0x00;
const TIM13_DIER_OFFSET : u32 = 0x0C;
const TIM13_SR_OFFSET : u32 = 0x10;
const TIM13_EGR_OFFSET : u32 = 0x14;
const TIM13_CCMR1_OFFSET : u32 = 0x18;
const TIM13_CCER_OFFSET : u32 = 0x20;
const TIM13_CNT_OFFSET : u32 = 0x24;
const TIM13_PSC_OFFSET : u32 = 0x28;
const TIM13_ARR_OFFSET : u32 = 0x2C;
const TIM13_CCR1_OFFSET : u32 = 0x34;
const TIM13_OR_OFFSET : u32 = 0x50;
    
        
const TIM14_CR1_OFFSET : u32 = 0x00;
const TIM14_DIER_OFFSET : u32 = 0x0C;
const TIM14_SR_OFFSET : u32 = 0x10;
const TIM14_EGR_OFFSET : u32 = 0x14;
const TIM14_CCMR1_OFFSET : u32 = 0x18;
const TIM14_CCER_OFFSET : u32 = 0x20;
const TIM14_CNT_OFFSET : u32 = 0x24;
const TIM14_PSC_OFFSET : u32 = 0x28;
const TIM14_ARR_OFFSET : u32 = 0x2C;
const TIM14_CCR1_OFFSET : u32 = 0x34;
const TIM14_OR_OFFSET : u32 = 0x50;
    
        
const TIM6_CR1_OFFSET : u32 = 0x00;
const TIM6_CR2_OFFSET : u32 = 0x04;
const TIM6_DIER_OFFSET : u32 = 0x0C;
const TIM6_SR_OFFSET : u32 = 0x10;
const TIM6_EGR_OFFSET : u32 = 0x14;
const TIM6_CNT_OFFSET : u32 = 0x24;
const TIM6_PSC_OFFSET : u32 = 0x28;
const TIM6_ARR_OFFSET : u32 = 0x2C;
    
        
const TIM7_CR1_OFFSET : u32 = 0x00;
const TIM7_CR2_OFFSET : u32 = 0x04;
const TIM7_DIER_OFFSET : u32 = 0x0C;
const TIM7_SR_OFFSET : u32 = 0x10;
const TIM7_EGR_OFFSET : u32 = 0x14;
const TIM7_CNT_OFFSET : u32 = 0x24;
const TIM7_PSC_OFFSET : u32 = 0x28;
const TIM7_ARR_OFFSET : u32 = 0x2C;
    
pub const TIM_CKD_CKINT : u32 = 0b00 << 8;
pub const TIM_CKD_CKINT2 : u32 = 0b01 << 8;
pub const TIM_CKD_CKINT4 : u32 = 0b10 << 8;
pub const TIM_ARPE : u32 = 1 << 7;
pub const TIM_CMS_MODE0 : u32 = 0b00 << 5;
pub const TIM_CMS_MODE1 : u32 = 0b01 << 5;
pub const TIM_CMS_MODE2 : u32 = 0b10 << 5;
pub const TIM_CMS_MODE3 : u32 = 0b11 << 5;
pub const TIM_DIR_DOWN : u32 = 1 << 4;
pub const TIM_OPM : u32 = 1 << 3;
pub const TIM_URS : u32 = 1 << 2;
pub const TIM_UDIS : u32 = 1 << 1;
pub const TIM_CEN : u32 = 1 << 0;
pub const TIM_TI1S : u32 = 1 << 7;
pub const TIM_MMS_RESET : u32 = 0b000 << 4;
pub const TIM_MMS_ENABLE : u32 = 0b001 << 4;
pub const TIM_MMS_UPDATE : u32 = 0b010 << 4;
pub const TIM_MMS_COMPARE_PULSE : u32 = 0b011 << 4;
pub const TIM_MMS_COMPARE1 : u32 = 0b100 << 4;
pub const TIM_MMS_COMPARE2 : u32 = 0b101 << 4;
pub const TIM_MMS_COMPARE3 : u32 = 0b110 << 4;
pub const TIM_MMS_COMPARE4 : u32 = 0b111 << 4;
pub const TIM_CCDSS : u32 = 1 << 3;
pub const TIM_ETP : u32 = 1 << 15;
pub const TIM_ECE : u32 = 1 << 14;
pub const TIM_ETPS_OFF : u32 = 0b00 << 12;
pub const TIM_ETPS_ON2 : u32 = 0b01 << 12;
pub const TIM_ETPS_ON4 : u32 = 0b10 << 12;
pub const TIM_ETPS_ON8 : u32 = 0b11 << 12;
pub const TIM_ETF_CKINT1 : u32 = 0b0000 << 8;
pub const TIM_ETF_CKINT2 : u32 = 0b0001 << 8;
pub const TIM_ETF_CKINT4 : u32 = 0b0010 << 8;
pub const TIM_ETF_CKINT8 : u32 = 0b0011 << 8;
pub const TIM_ETF_DTS2_6 : u32 = 0b0100 << 8;
pub const TIM_ETF_DTS2_8 : u32 = 0b0101 << 8;
pub const TIM_ETF_DTS4_6 : u32 = 0b0110 << 8;
pub const TIM_ETF_DTS4_8 : u32 = 0b0111 << 8;
pub const TIM_ETF_DTS8_6 : u32 = 0b1000 << 8;
pub const TIM_ETF_DTS8_8 : u32 = 0b1001 << 8;
pub const TIM_ETF_DTS16_5 : u32 = 0b1010 << 8;
pub const TIM_ETF_DTS16_6 : u32 = 0b1011 << 8;
pub const TIM_ETF_DTS16_8 : u32 = 0b1100 << 8;
pub const TIM_ETF_DTS32_5 : u32 = 0b1101 << 8;
pub const TIM_ETF_DTS32_6 : u32 = 0b1110 << 8;
pub const TIM_ETF_DTS32_8 : u32 = 0b1111 << 8;
pub const TIM_MSM : u32 = 1 << 7;
pub const TIM_TS_ITR0 : u32 = 0b000 << 4;
pub const TIM_TS_ITR1 : u32 = 0b001 << 4;
pub const TIM_TS_ITR2 : u32 = 0b010 << 4;
pub const TIM_TS_ITR3 : u32 = 0b011 << 4;
pub const TIM_TS_TI1F_ED : u32 = 0b100 << 4;
pub const TIM_TS_TI1FP1 : u32 = 0b101 << 4;
pub const TIM_TS_TI2FP2 : u32 = 0b110 << 4;
pub const TIM_TS_ETRF : u32 = 0b111 << 4;
pub const TIM_SMS_OFF : u32 = 0b000 << 0;
pub const TIM_SMS_ENC1 : u32 = 0b001 << 0;
pub const TIM_SMS_ENC2 : u32 = 0b010 << 0;
pub const TIM_SMS_ENC3 : u32 = 0b011 << 0;
pub const TIM_SMS_RESET : u32 = 0b100 << 0;
pub const TIM_SMS_GATED : u32 = 0b101 << 0;
pub const TIM_SMS_TRIGGER : u32 = 0b110 << 0;
pub const TIM_SMS_EXT : u32 = 0b111 << 0;
pub const TIM_TDE : u32 = 1 << 14;
pub const TIM_CC4DE : u32 = 1 << 13;
pub const TIM_CC3DE : u32 = 1 << 12;
pub const TIM_CC2DE : u32 = 1 << 11;
pub const TIM_CC1DE : u32 = 1 << 10;
pub const TIM_UDE : u32 = 1 << 9;
pub const TIM_TIE : u32 = 1 << 6;
pub const TIM_CC4IE : u32 = 1 << 4;
pub const TIM_CC3IE : u32 = 1 << 3;
pub const TIM_CC2IE : u32 = 1 << 2;
pub const TIM_CC1IE : u32 = 1 << 1;
pub const TIM_UIE : u32 = 1 << 0;
pub const TIM_CC4OF : u32 = 1 << 12;
pub const TIM_CC3OF : u32 = 1 << 11;
pub const TIM_CC2OF : u32 = 1 << 10;
pub const TIM_CC1OF : u32 = 1 << 9;
pub const TIM_TIF : u32 = 1 << 6;
pub const TIM_CC4IF : u32 = 1 << 4;
pub const TIM_CC3IF : u32 = 1 << 3;
pub const TIM_CC2IF : u32 = 1 << 2;
pub const TIM_CC1IF : u32 = 1 << 1;
pub const TIM_UIF : u32 = 1 << 0;
pub const TIM_CC4G : u32 = 1 << 4;
pub const TIM_CC3G : u32 = 1 << 3;
pub const TIM_CC2G : u32 = 1 << 2;
pub const TIM_CC1G : u32 = 1 << 1;
pub const TIM_UG : u32 = 1 << 0;
pub const TIM_OC2CE : u32 = 1 << 15;
pub const TIM_OC2M_FROZEN : u32 = 0b000 << 12;
pub const TIM_OC2M_SET : u32 = 0b001 << 12;
pub const TIM_OC2M_CLR : u32 = 0b010 << 12;
pub const TIM_OC2M_TOGGLE : u32 = 0b011 << 12;
pub const TIM_OC2M_PWM1 : u32 = 0b110 << 12;
pub const TIM_OC2M_PWM2 : u32 = 0b111 << 12;
pub const TIM_OC2PE : u32 = 1 << 11;
pub const TIM_OC2FE : u32 = 1 << 10;
pub const TIM_CCS2S_OUT : u32 = 0b00 << 8;
pub const TIM_CCS2S_IN2 : u32 = 0b01 << 8;
pub const TIM_CCS2S_IN1 : u32 = 0b10 << 8;
pub const TIM_CCS2S_TRC : u32 = 0b11 << 8;
pub const TIM_OC1CE : u32 = 1 << 7;
pub const TIM_OC1M_FROZEN : u32 = 0b000 << 4;
pub const TIM_OC1M_SET : u32 = 0b001 << 4;
pub const TIM_OC1M_CLR : u32 = 0b010 << 4;
pub const TIM_OC1M_TOGGLE : u32 = 0b011 << 4;
pub const TIM_OC1M_PWM1 : u32 = 0b110 << 4;
pub const TIM_OC1M_PWM2 : u32 = 0b111 << 4;
pub const TIM_OC1PE : u32 = 1 << 3;
pub const TIM_OC1FE : u32 = 1 << 2;
pub const TIM_CCS1S_OUT : u32 = 0b00 << 0;
pub const TIM_CCS1S_IN1 : u32 = 0b01 << 8;
pub const TIM_CCS1S_IN2 : u32 = 0b10 << 8;
pub const TIM_CCS1S_TRC : u32 = 0b11 << 8;
pub const TIM_OC4CE : u32 = 1 << 15;
pub const TIM_OC4M_FROZEN : u32 = 0b000 << 12;
pub const TIM_OC4M_SET : u32 = 0b001 << 12;
pub const TIM_OC4M_CLR : u32 = 0b010 << 12;
pub const TIM_OC4M_TOGGLE : u32 = 0b011 << 12;
pub const TIM_OC4M_PWM1 : u32 = 0b110 << 12;
pub const TIM_OC4M_PWM2 : u32 = 0b111 << 12;
pub const TIM_OC4PE : u32 = 1 << 11;
pub const TIM_OC4FE : u32 = 1 << 10;
pub const TIM_CCS4S_OUT : u32 = 0b00 << 8;
pub const TIM_CCS4S_IN3 : u32 = 0b01 << 8;
pub const TIM_CCS4S_IN4 : u32 = 0b10 << 8;
pub const TIM_CCS4S_TRC : u32 = 0b11 << 8;
pub const TIM_OC3CE : u32 = 1 << 7;
pub const TIM_OC3M_FROZEN : u32 = 0b000 << 4;
pub const TIM_OC3M_SET : u32 = 0b001 << 4;
pub const TIM_OC3M_CLR : u32 = 0b010 << 4;
pub const TIM_OC3M_TOGGLE : u32 = 0b011 << 4;
pub const TIM_OC3M_PWM1 : u32 = 0b110 << 4;
pub const TIM_OC3M_PWM2 : u32 = 0b111 << 4;
pub const TIM_OC3PE : u32 = 1 << 3;
pub const TIM_OC3RE : u32 = 1 << 2;
pub const TIM_CCS3S_OUT : u32 = 0b00 << 0;
pub const TIM_CCS3S_IN3 : u32 = 0b01 << 8;
pub const TIM_CCS3S_IN4 : u32 = 0b10 << 8;
pub const TIM_CCS3S_TRC : u32 = 0b11 << 8;
pub const TIM_CC4NP : u32 = 1 << 15;
pub const TIM_CC4P : u32 = 1 << 13;
pub const TIM_CC4E : u32 = 1 << 12;
pub const TIM_CC3NP : u32 = 1 << 11;
pub const TIM_CC3P : u32 = 1 << 9;
pub const TIM_CC3E : u32 = 1 << 8;
pub const TIM_CC2NP : u32 = 1 << 7;
pub const TIM_CC2P : u32 = 1 << 5;
pub const TIM_CC2E : u32 = 1 << 4;
pub const TIM_CC1NP : u32 = 1 << 3;
pub const TIM_CC1P : u32 = 1 << 1;
pub const TIM_CC1E : u32 = 1 << 0;
pub const TIM1_BRK_IRQ : u32 = 24;
pub const TIM1_UP_IRQ : u32 = 25;
pub const TIM1_TRG_COM_IRQ : u32 = 26;
pub const TIM1_CC_IRQ : u32 = 27;
pub const TIM2_IRQ : u32 = 28;
pub const TIM3_IRQ : u32 = 29;
pub const TIM4_IRQ : u32 = 30;
pub const TIM5_IRQ : u32 = 50;
pub const TIM7_IRQ : u32 = 55;
pub const TIM8_BRK_IRQ : u32 = 43;
pub const TIM8_UO_IRQ : u32 = 44;
pub const TIM8_TRG_COM_IRQ : u32 = 45;
pub const TIM8_CC_IRQ : u32 = 46;
pub const TIM9_IRQ : u32 = 24;
pub const TIM10_IRQ : u32 = 25;
pub const TIM11_IRQ : u32 = 26;
pub const TIM12_IRQ : u32 = 43;
pub const TIM13_IRQ : u32 = 44;
pub const TIM14_IRQ : u32 = 45;
        
pub fn tim1_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccmr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCMR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccr3_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCR3_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_ccr4_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_CCR4_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_btdr_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_BTDR_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_dcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_DCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim1_dmar_write(value: u32) {
    unsafe {
        write_volatile( (TIM1_ADR + TIM1_DMAR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim8_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccmr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCMR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccr3_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCR3_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_ccr4_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_CCR4_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_btdr_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_BTDR_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_dcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_DCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim8_dmar_write(value: u32) {
    unsafe {
        write_volatile( (TIM8_ADR + TIM8_DMAR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim2_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccmr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCMR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccr3_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCR3_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_ccr4_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_CCR4_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_dcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_DCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_dmar_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_DMAR_OFFSET) as *mut u32, value)
    };
}
pub fn tim2_or_write(value: u32) {
    unsafe {
        write_volatile( (TIM2_ADR + TIM2_OR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim3_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccmr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCMR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccr3_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCR3_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_ccr4_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_CCR4_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_dcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_DCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim3_dmar_write(value: u32) {
    unsafe {
        write_volatile( (TIM3_ADR + TIM3_DMAR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim4_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccmr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCMR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccr3_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCR3_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_ccr4_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_CCR4_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_dcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_DCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim4_dmar_write(value: u32) {
    unsafe {
        write_volatile( (TIM4_ADR + TIM4_DMAR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim5_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccmr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCMR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccr3_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCR3_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_ccr4_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_CCR4_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_dcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_DCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_dmar_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_DMAR_OFFSET) as *mut u32, value)
    };
}
pub fn tim5_or_write(value: u32) {
    unsafe {
        write_volatile( (TIM5_ADR + TIM5_OR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim9_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim9_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM9_ADR + TIM9_CCR2_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim12_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_smcr_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_SMCR_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim12_ccr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM12_ADR + TIM12_CCR2_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim10_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim10_or_write(value: u32) {
    unsafe {
        write_volatile( (TIM10_ADR + TIM10_OR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim11_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim11_or_write(value: u32) {
    unsafe {
        write_volatile( (TIM11_ADR + TIM11_OR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim13_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim13_or_write(value: u32) {
    unsafe {
        write_volatile( (TIM13_ADR + TIM13_OR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim14_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_ccmr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_CCMR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_ccer_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_CCER_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_ARR_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_ccr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_CCR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim14_or_write(value: u32) {
    unsafe {
        write_volatile( (TIM14_ADR + TIM14_OR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim6_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim6_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM6_ADR + TIM6_ARR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim7_cr1_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_cr2_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_dier_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_DIER_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_sr_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_SR_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_egr_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_EGR_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_cnt_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_CNT_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_psc_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_PSC_OFFSET) as *mut u32, value)
    };
}
pub fn tim7_arr_write(value: u32) {
    unsafe {
        write_volatile( (TIM7_ADR + TIM7_ARR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn tim1_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim1_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim1_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim1_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim1_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_SR_OFFSET) as *mut u32)
    }
}

pub fn tim1_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim1_ccmr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCMR2_OFFSET) as *mut u32)
    }
}
pub fn tim1_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim1_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim1_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim1_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim1_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim1_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCR2_OFFSET) as *mut u32)
    }
}
pub fn tim1_ccr3_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCR3_OFFSET) as *mut u32)
    }
}
pub fn tim1_ccr4_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_CCR4_OFFSET) as *mut u32)
    }
}
pub fn tim1_btdr_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_BTDR_OFFSET) as *mut u32)
    }
}
pub fn tim1_dcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_DCR_OFFSET) as *mut u32)
    }
}
pub fn tim1_dmar_read() -> u32 {
    unsafe {
        read_volatile( (TIM1_ADR + TIM1_DMAR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim8_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim8_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim8_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim8_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim8_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_SR_OFFSET) as *mut u32)
    }
}

pub fn tim8_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim8_ccmr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCMR2_OFFSET) as *mut u32)
    }
}
pub fn tim8_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim8_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim8_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim8_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim8_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim8_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCR2_OFFSET) as *mut u32)
    }
}
pub fn tim8_ccr3_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCR3_OFFSET) as *mut u32)
    }
}
pub fn tim8_ccr4_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_CCR4_OFFSET) as *mut u32)
    }
}
pub fn tim8_btdr_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_BTDR_OFFSET) as *mut u32)
    }
}
pub fn tim8_dcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_DCR_OFFSET) as *mut u32)
    }
}
pub fn tim8_dmar_read() -> u32 {
    unsafe {
        read_volatile( (TIM8_ADR + TIM8_DMAR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim2_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim2_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim2_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim2_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim2_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_SR_OFFSET) as *mut u32)
    }
}

pub fn tim2_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim2_ccmr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCMR2_OFFSET) as *mut u32)
    }
}
pub fn tim2_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim2_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim2_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim2_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim2_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim2_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCR2_OFFSET) as *mut u32)
    }
}
pub fn tim2_ccr3_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCR3_OFFSET) as *mut u32)
    }
}
pub fn tim2_ccr4_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_CCR4_OFFSET) as *mut u32)
    }
}
pub fn tim2_dcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_DCR_OFFSET) as *mut u32)
    }
}
pub fn tim2_dmar_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_DMAR_OFFSET) as *mut u32)
    }
}
pub fn tim2_or_read() -> u32 {
    unsafe {
        read_volatile( (TIM2_ADR + TIM2_OR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim3_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim3_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim3_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim3_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim3_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_SR_OFFSET) as *mut u32)
    }
}

pub fn tim3_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim3_ccmr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCMR2_OFFSET) as *mut u32)
    }
}
pub fn tim3_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim3_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim3_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim3_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim3_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim3_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCR2_OFFSET) as *mut u32)
    }
}
pub fn tim3_ccr3_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCR3_OFFSET) as *mut u32)
    }
}
pub fn tim3_ccr4_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_CCR4_OFFSET) as *mut u32)
    }
}
pub fn tim3_dcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_DCR_OFFSET) as *mut u32)
    }
}
pub fn tim3_dmar_read() -> u32 {
    unsafe {
        read_volatile( (TIM3_ADR + TIM3_DMAR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim4_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim4_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim4_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim4_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim4_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_SR_OFFSET) as *mut u32)
    }
}

pub fn tim4_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim4_ccmr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCMR2_OFFSET) as *mut u32)
    }
}
pub fn tim4_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim4_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim4_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim4_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim4_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim4_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCR2_OFFSET) as *mut u32)
    }
}
pub fn tim4_ccr3_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCR3_OFFSET) as *mut u32)
    }
}
pub fn tim4_ccr4_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_CCR4_OFFSET) as *mut u32)
    }
}
pub fn tim4_dcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_DCR_OFFSET) as *mut u32)
    }
}
pub fn tim4_dmar_read() -> u32 {
    unsafe {
        read_volatile( (TIM4_ADR + TIM4_DMAR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim5_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim5_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim5_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim5_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim5_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_SR_OFFSET) as *mut u32)
    }
}

pub fn tim5_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim5_ccmr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCMR2_OFFSET) as *mut u32)
    }
}
pub fn tim5_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim5_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim5_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim5_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim5_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim5_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCR2_OFFSET) as *mut u32)
    }
}
pub fn tim5_ccr3_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCR3_OFFSET) as *mut u32)
    }
}
pub fn tim5_ccr4_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_CCR4_OFFSET) as *mut u32)
    }
}
pub fn tim5_dcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_DCR_OFFSET) as *mut u32)
    }
}
pub fn tim5_dmar_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_DMAR_OFFSET) as *mut u32)
    }
}
pub fn tim5_or_read() -> u32 {
    unsafe {
        read_volatile( (TIM5_ADR + TIM5_OR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim9_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim9_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim9_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim9_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_SR_OFFSET) as *mut u32)
    }
}

pub fn tim9_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim9_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim9_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim9_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim9_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim9_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim9_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM9_ADR + TIM9_CCR2_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim12_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim12_smcr_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_SMCR_OFFSET) as *mut u32)
    }
}
pub fn tim12_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim12_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_SR_OFFSET) as *mut u32)
    }
}

pub fn tim12_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim12_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim12_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim12_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim12_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim12_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim12_ccr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM12_ADR + TIM12_CCR2_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim10_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim10_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim10_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_SR_OFFSET) as *mut u32)
    }
}

pub fn tim10_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim10_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim10_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim10_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim10_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim10_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim10_or_read() -> u32 {
    unsafe {
        read_volatile( (TIM10_ADR + TIM10_OR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim11_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim11_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim11_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_SR_OFFSET) as *mut u32)
    }
}

pub fn tim11_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim11_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim11_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim11_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim11_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim11_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim11_or_read() -> u32 {
    unsafe {
        read_volatile( (TIM11_ADR + TIM11_OR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim13_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim13_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim13_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_SR_OFFSET) as *mut u32)
    }
}

pub fn tim13_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim13_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim13_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim13_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim13_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim13_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim13_or_read() -> u32 {
    unsafe {
        read_volatile( (TIM13_ADR + TIM13_OR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim14_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim14_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim14_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_SR_OFFSET) as *mut u32)
    }
}

pub fn tim14_ccmr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_CCMR1_OFFSET) as *mut u32)
    }
}
pub fn tim14_ccer_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_CCER_OFFSET) as *mut u32)
    }
}
pub fn tim14_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim14_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim14_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_ARR_OFFSET) as *mut u32)
    }
}
pub fn tim14_ccr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_CCR1_OFFSET) as *mut u32)
    }
}
pub fn tim14_or_read() -> u32 {
    unsafe {
        read_volatile( (TIM14_ADR + TIM14_OR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim6_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim6_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim6_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim6_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_SR_OFFSET) as *mut u32)
    }
}

pub fn tim6_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim6_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim6_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM6_ADR + TIM6_ARR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim7_cr1_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_CR1_OFFSET) as *mut u32)
    }
}
pub fn tim7_cr2_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_CR2_OFFSET) as *mut u32)
    }
}
pub fn tim7_dier_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_DIER_OFFSET) as *mut u32)
    }
}
pub fn tim7_sr_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_SR_OFFSET) as *mut u32)
    }
}

pub fn tim7_cnt_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_CNT_OFFSET) as *mut u32)
    }
}
pub fn tim7_psc_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_PSC_OFFSET) as *mut u32)
    }
}
pub fn tim7_arr_read() -> u32 {
    unsafe {
        read_volatile( (TIM7_ADR + TIM7_ARR_OFFSET) as *mut u32)
    }
}
    
        
pub fn tim1_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_cr1_write(rep_bits(tim1_cr1_read(), position, size, value));
}
pub fn tim1_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_cr2_write(rep_bits(tim1_cr2_read(), position, size, value));
}
pub fn tim1_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_smcr_write(rep_bits(tim1_smcr_read(), position, size, value));
}
pub fn tim1_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_dier_write(rep_bits(tim1_dier_read(), position, size, value));
}
pub fn tim1_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_sr_write(rep_bits(tim1_sr_read(), position, size, value));
}

pub fn tim1_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccmr1_write(rep_bits(tim1_ccmr1_read(), position, size, value));
}
pub fn tim1_ccmr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccmr2_write(rep_bits(tim1_ccmr2_read(), position, size, value));
}
pub fn tim1_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccer_write(rep_bits(tim1_ccer_read(), position, size, value));
}
pub fn tim1_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_cnt_write(rep_bits(tim1_cnt_read(), position, size, value));
}
pub fn tim1_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_psc_write(rep_bits(tim1_psc_read(), position, size, value));
}
pub fn tim1_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_arr_write(rep_bits(tim1_arr_read(), position, size, value));
}
pub fn tim1_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccr1_write(rep_bits(tim1_ccr1_read(), position, size, value));
}
pub fn tim1_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccr2_write(rep_bits(tim1_ccr2_read(), position, size, value));
}
pub fn tim1_ccr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccr3_write(rep_bits(tim1_ccr3_read(), position, size, value));
}
pub fn tim1_ccr4_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_ccr4_write(rep_bits(tim1_ccr4_read(), position, size, value));
}
pub fn tim1_btdr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_btdr_write(rep_bits(tim1_btdr_read(), position, size, value));
}
pub fn tim1_dcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_dcr_write(rep_bits(tim1_dcr_read(), position, size, value));
}
pub fn tim1_dmar_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim1_dmar_write(rep_bits(tim1_dmar_read(), position, size, value));
}
    
        
pub fn tim8_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_cr1_write(rep_bits(tim8_cr1_read(), position, size, value));
}
pub fn tim8_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_cr2_write(rep_bits(tim8_cr2_read(), position, size, value));
}
pub fn tim8_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_smcr_write(rep_bits(tim8_smcr_read(), position, size, value));
}
pub fn tim8_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_dier_write(rep_bits(tim8_dier_read(), position, size, value));
}
pub fn tim8_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_sr_write(rep_bits(tim8_sr_read(), position, size, value));
}

pub fn tim8_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccmr1_write(rep_bits(tim8_ccmr1_read(), position, size, value));
}
pub fn tim8_ccmr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccmr2_write(rep_bits(tim8_ccmr2_read(), position, size, value));
}
pub fn tim8_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccer_write(rep_bits(tim8_ccer_read(), position, size, value));
}
pub fn tim8_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_cnt_write(rep_bits(tim8_cnt_read(), position, size, value));
}
pub fn tim8_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_psc_write(rep_bits(tim8_psc_read(), position, size, value));
}
pub fn tim8_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_arr_write(rep_bits(tim8_arr_read(), position, size, value));
}
pub fn tim8_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccr1_write(rep_bits(tim8_ccr1_read(), position, size, value));
}
pub fn tim8_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccr2_write(rep_bits(tim8_ccr2_read(), position, size, value));
}
pub fn tim8_ccr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccr3_write(rep_bits(tim8_ccr3_read(), position, size, value));
}
pub fn tim8_ccr4_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_ccr4_write(rep_bits(tim8_ccr4_read(), position, size, value));
}
pub fn tim8_btdr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_btdr_write(rep_bits(tim8_btdr_read(), position, size, value));
}
pub fn tim8_dcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_dcr_write(rep_bits(tim8_dcr_read(), position, size, value));
}
pub fn tim8_dmar_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim8_dmar_write(rep_bits(tim8_dmar_read(), position, size, value));
}
    
        
pub fn tim2_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_cr1_write(rep_bits(tim2_cr1_read(), position, size, value));
}
pub fn tim2_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_cr2_write(rep_bits(tim2_cr2_read(), position, size, value));
}
pub fn tim2_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_smcr_write(rep_bits(tim2_smcr_read(), position, size, value));
}
pub fn tim2_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_dier_write(rep_bits(tim2_dier_read(), position, size, value));
}
pub fn tim2_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_sr_write(rep_bits(tim2_sr_read(), position, size, value));
}

pub fn tim2_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccmr1_write(rep_bits(tim2_ccmr1_read(), position, size, value));
}
pub fn tim2_ccmr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccmr2_write(rep_bits(tim2_ccmr2_read(), position, size, value));
}
pub fn tim2_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccer_write(rep_bits(tim2_ccer_read(), position, size, value));
}
pub fn tim2_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_cnt_write(rep_bits(tim2_cnt_read(), position, size, value));
}
pub fn tim2_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_psc_write(rep_bits(tim2_psc_read(), position, size, value));
}
pub fn tim2_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_arr_write(rep_bits(tim2_arr_read(), position, size, value));
}
pub fn tim2_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccr1_write(rep_bits(tim2_ccr1_read(), position, size, value));
}
pub fn tim2_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccr2_write(rep_bits(tim2_ccr2_read(), position, size, value));
}
pub fn tim2_ccr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccr3_write(rep_bits(tim2_ccr3_read(), position, size, value));
}
pub fn tim2_ccr4_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_ccr4_write(rep_bits(tim2_ccr4_read(), position, size, value));
}
pub fn tim2_dcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_dcr_write(rep_bits(tim2_dcr_read(), position, size, value));
}
pub fn tim2_dmar_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_dmar_write(rep_bits(tim2_dmar_read(), position, size, value));
}
pub fn tim2_or_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim2_or_write(rep_bits(tim2_or_read(), position, size, value));
}
    
        
pub fn tim3_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_cr1_write(rep_bits(tim3_cr1_read(), position, size, value));
}
pub fn tim3_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_cr2_write(rep_bits(tim3_cr2_read(), position, size, value));
}
pub fn tim3_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_smcr_write(rep_bits(tim3_smcr_read(), position, size, value));
}
pub fn tim3_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_dier_write(rep_bits(tim3_dier_read(), position, size, value));
}
pub fn tim3_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_sr_write(rep_bits(tim3_sr_read(), position, size, value));
}

pub fn tim3_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccmr1_write(rep_bits(tim3_ccmr1_read(), position, size, value));
}
pub fn tim3_ccmr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccmr2_write(rep_bits(tim3_ccmr2_read(), position, size, value));
}
pub fn tim3_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccer_write(rep_bits(tim3_ccer_read(), position, size, value));
}
pub fn tim3_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_cnt_write(rep_bits(tim3_cnt_read(), position, size, value));
}
pub fn tim3_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_psc_write(rep_bits(tim3_psc_read(), position, size, value));
}
pub fn tim3_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_arr_write(rep_bits(tim3_arr_read(), position, size, value));
}
pub fn tim3_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccr1_write(rep_bits(tim3_ccr1_read(), position, size, value));
}
pub fn tim3_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccr2_write(rep_bits(tim3_ccr2_read(), position, size, value));
}
pub fn tim3_ccr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccr3_write(rep_bits(tim3_ccr3_read(), position, size, value));
}
pub fn tim3_ccr4_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_ccr4_write(rep_bits(tim3_ccr4_read(), position, size, value));
}
pub fn tim3_dcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_dcr_write(rep_bits(tim3_dcr_read(), position, size, value));
}
pub fn tim3_dmar_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim3_dmar_write(rep_bits(tim3_dmar_read(), position, size, value));
}
    
        
pub fn tim4_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_cr1_write(rep_bits(tim4_cr1_read(), position, size, value));
}
pub fn tim4_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_cr2_write(rep_bits(tim4_cr2_read(), position, size, value));
}
pub fn tim4_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_smcr_write(rep_bits(tim4_smcr_read(), position, size, value));
}
pub fn tim4_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_dier_write(rep_bits(tim4_dier_read(), position, size, value));
}
pub fn tim4_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_sr_write(rep_bits(tim4_sr_read(), position, size, value));
}

pub fn tim4_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccmr1_write(rep_bits(tim4_ccmr1_read(), position, size, value));
}
pub fn tim4_ccmr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccmr2_write(rep_bits(tim4_ccmr2_read(), position, size, value));
}
pub fn tim4_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccer_write(rep_bits(tim4_ccer_read(), position, size, value));
}
pub fn tim4_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_cnt_write(rep_bits(tim4_cnt_read(), position, size, value));
}
pub fn tim4_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_psc_write(rep_bits(tim4_psc_read(), position, size, value));
}
pub fn tim4_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_arr_write(rep_bits(tim4_arr_read(), position, size, value));
}
pub fn tim4_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccr1_write(rep_bits(tim4_ccr1_read(), position, size, value));
}
pub fn tim4_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccr2_write(rep_bits(tim4_ccr2_read(), position, size, value));
}
pub fn tim4_ccr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccr3_write(rep_bits(tim4_ccr3_read(), position, size, value));
}
pub fn tim4_ccr4_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_ccr4_write(rep_bits(tim4_ccr4_read(), position, size, value));
}
pub fn tim4_dcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_dcr_write(rep_bits(tim4_dcr_read(), position, size, value));
}
pub fn tim4_dmar_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim4_dmar_write(rep_bits(tim4_dmar_read(), position, size, value));
}
    
        
pub fn tim5_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_cr1_write(rep_bits(tim5_cr1_read(), position, size, value));
}
pub fn tim5_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_cr2_write(rep_bits(tim5_cr2_read(), position, size, value));
}
pub fn tim5_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_smcr_write(rep_bits(tim5_smcr_read(), position, size, value));
}
pub fn tim5_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_dier_write(rep_bits(tim5_dier_read(), position, size, value));
}
pub fn tim5_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_sr_write(rep_bits(tim5_sr_read(), position, size, value));
}

pub fn tim5_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccmr1_write(rep_bits(tim5_ccmr1_read(), position, size, value));
}
pub fn tim5_ccmr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccmr2_write(rep_bits(tim5_ccmr2_read(), position, size, value));
}
pub fn tim5_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccer_write(rep_bits(tim5_ccer_read(), position, size, value));
}
pub fn tim5_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_cnt_write(rep_bits(tim5_cnt_read(), position, size, value));
}
pub fn tim5_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_psc_write(rep_bits(tim5_psc_read(), position, size, value));
}
pub fn tim5_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_arr_write(rep_bits(tim5_arr_read(), position, size, value));
}
pub fn tim5_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccr1_write(rep_bits(tim5_ccr1_read(), position, size, value));
}
pub fn tim5_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccr2_write(rep_bits(tim5_ccr2_read(), position, size, value));
}
pub fn tim5_ccr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccr3_write(rep_bits(tim5_ccr3_read(), position, size, value));
}
pub fn tim5_ccr4_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_ccr4_write(rep_bits(tim5_ccr4_read(), position, size, value));
}
pub fn tim5_dcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_dcr_write(rep_bits(tim5_dcr_read(), position, size, value));
}
pub fn tim5_dmar_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_dmar_write(rep_bits(tim5_dmar_read(), position, size, value));
}
pub fn tim5_or_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim5_or_write(rep_bits(tim5_or_read(), position, size, value));
}
    
        
pub fn tim9_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_cr1_write(rep_bits(tim9_cr1_read(), position, size, value));
}
pub fn tim9_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_smcr_write(rep_bits(tim9_smcr_read(), position, size, value));
}
pub fn tim9_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_dier_write(rep_bits(tim9_dier_read(), position, size, value));
}
pub fn tim9_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_sr_write(rep_bits(tim9_sr_read(), position, size, value));
}

pub fn tim9_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_ccmr1_write(rep_bits(tim9_ccmr1_read(), position, size, value));
}
pub fn tim9_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_ccer_write(rep_bits(tim9_ccer_read(), position, size, value));
}
pub fn tim9_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_cnt_write(rep_bits(tim9_cnt_read(), position, size, value));
}
pub fn tim9_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_psc_write(rep_bits(tim9_psc_read(), position, size, value));
}
pub fn tim9_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_arr_write(rep_bits(tim9_arr_read(), position, size, value));
}
pub fn tim9_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_ccr1_write(rep_bits(tim9_ccr1_read(), position, size, value));
}
pub fn tim9_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim9_ccr2_write(rep_bits(tim9_ccr2_read(), position, size, value));
}
    
        
pub fn tim12_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_cr1_write(rep_bits(tim12_cr1_read(), position, size, value));
}
pub fn tim12_smcr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_smcr_write(rep_bits(tim12_smcr_read(), position, size, value));
}
pub fn tim12_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_dier_write(rep_bits(tim12_dier_read(), position, size, value));
}
pub fn tim12_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_sr_write(rep_bits(tim12_sr_read(), position, size, value));
}

pub fn tim12_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_ccmr1_write(rep_bits(tim12_ccmr1_read(), position, size, value));
}
pub fn tim12_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_ccer_write(rep_bits(tim12_ccer_read(), position, size, value));
}
pub fn tim12_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_cnt_write(rep_bits(tim12_cnt_read(), position, size, value));
}
pub fn tim12_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_psc_write(rep_bits(tim12_psc_read(), position, size, value));
}
pub fn tim12_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_arr_write(rep_bits(tim12_arr_read(), position, size, value));
}
pub fn tim12_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_ccr1_write(rep_bits(tim12_ccr1_read(), position, size, value));
}
pub fn tim12_ccr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim12_ccr2_write(rep_bits(tim12_ccr2_read(), position, size, value));
}
    
        
pub fn tim10_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_cr1_write(rep_bits(tim10_cr1_read(), position, size, value));
}
pub fn tim10_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_dier_write(rep_bits(tim10_dier_read(), position, size, value));
}
pub fn tim10_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_sr_write(rep_bits(tim10_sr_read(), position, size, value));
}

pub fn tim10_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_ccmr1_write(rep_bits(tim10_ccmr1_read(), position, size, value));
}
pub fn tim10_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_ccer_write(rep_bits(tim10_ccer_read(), position, size, value));
}
pub fn tim10_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_cnt_write(rep_bits(tim10_cnt_read(), position, size, value));
}
pub fn tim10_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_psc_write(rep_bits(tim10_psc_read(), position, size, value));
}
pub fn tim10_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_arr_write(rep_bits(tim10_arr_read(), position, size, value));
}
pub fn tim10_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_ccr1_write(rep_bits(tim10_ccr1_read(), position, size, value));
}
pub fn tim10_or_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim10_or_write(rep_bits(tim10_or_read(), position, size, value));
}
    
        
pub fn tim11_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_cr1_write(rep_bits(tim11_cr1_read(), position, size, value));
}
pub fn tim11_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_dier_write(rep_bits(tim11_dier_read(), position, size, value));
}
pub fn tim11_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_sr_write(rep_bits(tim11_sr_read(), position, size, value));
}

pub fn tim11_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_ccmr1_write(rep_bits(tim11_ccmr1_read(), position, size, value));
}
pub fn tim11_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_ccer_write(rep_bits(tim11_ccer_read(), position, size, value));
}
pub fn tim11_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_cnt_write(rep_bits(tim11_cnt_read(), position, size, value));
}
pub fn tim11_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_psc_write(rep_bits(tim11_psc_read(), position, size, value));
}
pub fn tim11_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_arr_write(rep_bits(tim11_arr_read(), position, size, value));
}
pub fn tim11_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_ccr1_write(rep_bits(tim11_ccr1_read(), position, size, value));
}
pub fn tim11_or_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim11_or_write(rep_bits(tim11_or_read(), position, size, value));
}
    
        
pub fn tim13_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_cr1_write(rep_bits(tim13_cr1_read(), position, size, value));
}
pub fn tim13_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_dier_write(rep_bits(tim13_dier_read(), position, size, value));
}
pub fn tim13_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_sr_write(rep_bits(tim13_sr_read(), position, size, value));
}

pub fn tim13_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_ccmr1_write(rep_bits(tim13_ccmr1_read(), position, size, value));
}
pub fn tim13_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_ccer_write(rep_bits(tim13_ccer_read(), position, size, value));
}
pub fn tim13_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_cnt_write(rep_bits(tim13_cnt_read(), position, size, value));
}
pub fn tim13_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_psc_write(rep_bits(tim13_psc_read(), position, size, value));
}
pub fn tim13_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_arr_write(rep_bits(tim13_arr_read(), position, size, value));
}
pub fn tim13_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_ccr1_write(rep_bits(tim13_ccr1_read(), position, size, value));
}
pub fn tim13_or_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim13_or_write(rep_bits(tim13_or_read(), position, size, value));
}
    
        
pub fn tim14_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_cr1_write(rep_bits(tim14_cr1_read(), position, size, value));
}
pub fn tim14_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_dier_write(rep_bits(tim14_dier_read(), position, size, value));
}
pub fn tim14_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_sr_write(rep_bits(tim14_sr_read(), position, size, value));
}

pub fn tim14_ccmr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_ccmr1_write(rep_bits(tim14_ccmr1_read(), position, size, value));
}
pub fn tim14_ccer_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_ccer_write(rep_bits(tim14_ccer_read(), position, size, value));
}
pub fn tim14_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_cnt_write(rep_bits(tim14_cnt_read(), position, size, value));
}
pub fn tim14_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_psc_write(rep_bits(tim14_psc_read(), position, size, value));
}
pub fn tim14_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_arr_write(rep_bits(tim14_arr_read(), position, size, value));
}
pub fn tim14_ccr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_ccr1_write(rep_bits(tim14_ccr1_read(), position, size, value));
}
pub fn tim14_or_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim14_or_write(rep_bits(tim14_or_read(), position, size, value));
}
    
        
pub fn tim6_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_cr1_write(rep_bits(tim6_cr1_read(), position, size, value));
}
pub fn tim6_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_cr2_write(rep_bits(tim6_cr2_read(), position, size, value));
}
pub fn tim6_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_dier_write(rep_bits(tim6_dier_read(), position, size, value));
}
pub fn tim6_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_sr_write(rep_bits(tim6_sr_read(), position, size, value));
}

pub fn tim6_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_cnt_write(rep_bits(tim6_cnt_read(), position, size, value));
}
pub fn tim6_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_psc_write(rep_bits(tim6_psc_read(), position, size, value));
}
pub fn tim6_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim6_arr_write(rep_bits(tim6_arr_read(), position, size, value));
}
    
        
pub fn tim7_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_cr1_write(rep_bits(tim7_cr1_read(), position, size, value));
}
pub fn tim7_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_cr2_write(rep_bits(tim7_cr2_read(), position, size, value));
}
pub fn tim7_dier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_dier_write(rep_bits(tim7_dier_read(), position, size, value));
}
pub fn tim7_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_sr_write(rep_bits(tim7_sr_read(), position, size, value));
}

pub fn tim7_cnt_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_cnt_write(rep_bits(tim7_cnt_read(), position, size, value));
}
pub fn tim7_psc_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_psc_write(rep_bits(tim7_psc_read(), position, size, value));
}
pub fn tim7_arr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    tim7_arr_write(rep_bits(tim7_arr_read(), position, size, value));
}
    
        
pub fn tim1_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_cr1_write(tim1_cr1_read() | value),
        31 => tim1_cr1_write(tim1_cr1_read() & value),
        _ => (),
    }


}
pub fn tim1_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_cr2_write(tim1_cr2_read() | value),
        31 => tim1_cr2_write(tim1_cr2_read() & value),
        _ => (),
    }


}
pub fn tim1_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_smcr_write(tim1_smcr_read() | value),
        31 => tim1_smcr_write(tim1_smcr_read() & value),
        _ => (),
    }


}
pub fn tim1_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_dier_write(tim1_dier_read() | value),
        31 => tim1_dier_write(tim1_dier_read() & value),
        _ => (),
    }


}
pub fn tim1_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_sr_write(tim1_sr_read() | value),
        31 => tim1_sr_write(tim1_sr_read() & value),
        _ => (),
    }


}

pub fn tim1_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccmr1_write(tim1_ccmr1_read() | value),
        31 => tim1_ccmr1_write(tim1_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim1_ccmr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccmr2_write(tim1_ccmr2_read() | value),
        31 => tim1_ccmr2_write(tim1_ccmr2_read() & value),
        _ => (),
    }


}
pub fn tim1_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccer_write(tim1_ccer_read() | value),
        31 => tim1_ccer_write(tim1_ccer_read() & value),
        _ => (),
    }


}
pub fn tim1_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_cnt_write(tim1_cnt_read() | value),
        31 => tim1_cnt_write(tim1_cnt_read() & value),
        _ => (),
    }


}
pub fn tim1_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_psc_write(tim1_psc_read() | value),
        31 => tim1_psc_write(tim1_psc_read() & value),
        _ => (),
    }


}
pub fn tim1_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_arr_write(tim1_arr_read() | value),
        31 => tim1_arr_write(tim1_arr_read() & value),
        _ => (),
    }


}
pub fn tim1_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccr1_write(tim1_ccr1_read() | value),
        31 => tim1_ccr1_write(tim1_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim1_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccr2_write(tim1_ccr2_read() | value),
        31 => tim1_ccr2_write(tim1_ccr2_read() & value),
        _ => (),
    }


}
pub fn tim1_ccr3_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccr3_write(tim1_ccr3_read() | value),
        31 => tim1_ccr3_write(tim1_ccr3_read() & value),
        _ => (),
    }


}
pub fn tim1_ccr4_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_ccr4_write(tim1_ccr4_read() | value),
        31 => tim1_ccr4_write(tim1_ccr4_read() & value),
        _ => (),
    }


}
pub fn tim1_btdr_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_btdr_write(tim1_btdr_read() | value),
        31 => tim1_btdr_write(tim1_btdr_read() & value),
        _ => (),
    }


}
pub fn tim1_dcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_dcr_write(tim1_dcr_read() | value),
        31 => tim1_dcr_write(tim1_dcr_read() & value),
        _ => (),
    }


}
pub fn tim1_dmar_seti(value: u32) {
    match value.count_ones() {
        1 => tim1_dmar_write(tim1_dmar_read() | value),
        31 => tim1_dmar_write(tim1_dmar_read() & value),
        _ => (),
    }


}
    
        
pub fn tim8_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_cr1_write(tim8_cr1_read() | value),
        31 => tim8_cr1_write(tim8_cr1_read() & value),
        _ => (),
    }


}
pub fn tim8_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_cr2_write(tim8_cr2_read() | value),
        31 => tim8_cr2_write(tim8_cr2_read() & value),
        _ => (),
    }


}
pub fn tim8_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_smcr_write(tim8_smcr_read() | value),
        31 => tim8_smcr_write(tim8_smcr_read() & value),
        _ => (),
    }


}
pub fn tim8_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_dier_write(tim8_dier_read() | value),
        31 => tim8_dier_write(tim8_dier_read() & value),
        _ => (),
    }


}
pub fn tim8_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_sr_write(tim8_sr_read() | value),
        31 => tim8_sr_write(tim8_sr_read() & value),
        _ => (),
    }


}

pub fn tim8_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccmr1_write(tim8_ccmr1_read() | value),
        31 => tim8_ccmr1_write(tim8_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim8_ccmr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccmr2_write(tim8_ccmr2_read() | value),
        31 => tim8_ccmr2_write(tim8_ccmr2_read() & value),
        _ => (),
    }


}
pub fn tim8_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccer_write(tim8_ccer_read() | value),
        31 => tim8_ccer_write(tim8_ccer_read() & value),
        _ => (),
    }


}
pub fn tim8_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_cnt_write(tim8_cnt_read() | value),
        31 => tim8_cnt_write(tim8_cnt_read() & value),
        _ => (),
    }


}
pub fn tim8_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_psc_write(tim8_psc_read() | value),
        31 => tim8_psc_write(tim8_psc_read() & value),
        _ => (),
    }


}
pub fn tim8_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_arr_write(tim8_arr_read() | value),
        31 => tim8_arr_write(tim8_arr_read() & value),
        _ => (),
    }


}
pub fn tim8_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccr1_write(tim8_ccr1_read() | value),
        31 => tim8_ccr1_write(tim8_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim8_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccr2_write(tim8_ccr2_read() | value),
        31 => tim8_ccr2_write(tim8_ccr2_read() & value),
        _ => (),
    }


}
pub fn tim8_ccr3_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccr3_write(tim8_ccr3_read() | value),
        31 => tim8_ccr3_write(tim8_ccr3_read() & value),
        _ => (),
    }


}
pub fn tim8_ccr4_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_ccr4_write(tim8_ccr4_read() | value),
        31 => tim8_ccr4_write(tim8_ccr4_read() & value),
        _ => (),
    }


}
pub fn tim8_btdr_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_btdr_write(tim8_btdr_read() | value),
        31 => tim8_btdr_write(tim8_btdr_read() & value),
        _ => (),
    }


}
pub fn tim8_dcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_dcr_write(tim8_dcr_read() | value),
        31 => tim8_dcr_write(tim8_dcr_read() & value),
        _ => (),
    }


}
pub fn tim8_dmar_seti(value: u32) {
    match value.count_ones() {
        1 => tim8_dmar_write(tim8_dmar_read() | value),
        31 => tim8_dmar_write(tim8_dmar_read() & value),
        _ => (),
    }


}
    
        
pub fn tim2_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_cr1_write(tim2_cr1_read() | value),
        31 => tim2_cr1_write(tim2_cr1_read() & value),
        _ => (),
    }


}
pub fn tim2_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_cr2_write(tim2_cr2_read() | value),
        31 => tim2_cr2_write(tim2_cr2_read() & value),
        _ => (),
    }


}
pub fn tim2_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_smcr_write(tim2_smcr_read() | value),
        31 => tim2_smcr_write(tim2_smcr_read() & value),
        _ => (),
    }


}
pub fn tim2_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_dier_write(tim2_dier_read() | value),
        31 => tim2_dier_write(tim2_dier_read() & value),
        _ => (),
    }


}
pub fn tim2_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_sr_write(tim2_sr_read() | value),
        31 => tim2_sr_write(tim2_sr_read() & value),
        _ => (),
    }


}

pub fn tim2_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccmr1_write(tim2_ccmr1_read() | value),
        31 => tim2_ccmr1_write(tim2_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim2_ccmr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccmr2_write(tim2_ccmr2_read() | value),
        31 => tim2_ccmr2_write(tim2_ccmr2_read() & value),
        _ => (),
    }


}
pub fn tim2_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccer_write(tim2_ccer_read() | value),
        31 => tim2_ccer_write(tim2_ccer_read() & value),
        _ => (),
    }


}
pub fn tim2_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_cnt_write(tim2_cnt_read() | value),
        31 => tim2_cnt_write(tim2_cnt_read() & value),
        _ => (),
    }


}
pub fn tim2_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_psc_write(tim2_psc_read() | value),
        31 => tim2_psc_write(tim2_psc_read() & value),
        _ => (),
    }


}
pub fn tim2_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_arr_write(tim2_arr_read() | value),
        31 => tim2_arr_write(tim2_arr_read() & value),
        _ => (),
    }


}
pub fn tim2_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccr1_write(tim2_ccr1_read() | value),
        31 => tim2_ccr1_write(tim2_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim2_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccr2_write(tim2_ccr2_read() | value),
        31 => tim2_ccr2_write(tim2_ccr2_read() & value),
        _ => (),
    }


}
pub fn tim2_ccr3_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccr3_write(tim2_ccr3_read() | value),
        31 => tim2_ccr3_write(tim2_ccr3_read() & value),
        _ => (),
    }


}
pub fn tim2_ccr4_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_ccr4_write(tim2_ccr4_read() | value),
        31 => tim2_ccr4_write(tim2_ccr4_read() & value),
        _ => (),
    }


}
pub fn tim2_dcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_dcr_write(tim2_dcr_read() | value),
        31 => tim2_dcr_write(tim2_dcr_read() & value),
        _ => (),
    }


}
pub fn tim2_dmar_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_dmar_write(tim2_dmar_read() | value),
        31 => tim2_dmar_write(tim2_dmar_read() & value),
        _ => (),
    }


}
pub fn tim2_or_seti(value: u32) {
    match value.count_ones() {
        1 => tim2_or_write(tim2_or_read() | value),
        31 => tim2_or_write(tim2_or_read() & value),
        _ => (),
    }


}
    
        
pub fn tim3_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_cr1_write(tim3_cr1_read() | value),
        31 => tim3_cr1_write(tim3_cr1_read() & value),
        _ => (),
    }


}
pub fn tim3_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_cr2_write(tim3_cr2_read() | value),
        31 => tim3_cr2_write(tim3_cr2_read() & value),
        _ => (),
    }


}
pub fn tim3_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_smcr_write(tim3_smcr_read() | value),
        31 => tim3_smcr_write(tim3_smcr_read() & value),
        _ => (),
    }


}
pub fn tim3_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_dier_write(tim3_dier_read() | value),
        31 => tim3_dier_write(tim3_dier_read() & value),
        _ => (),
    }


}
pub fn tim3_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_sr_write(tim3_sr_read() | value),
        31 => tim3_sr_write(tim3_sr_read() & value),
        _ => (),
    }


}

pub fn tim3_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccmr1_write(tim3_ccmr1_read() | value),
        31 => tim3_ccmr1_write(tim3_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim3_ccmr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccmr2_write(tim3_ccmr2_read() | value),
        31 => tim3_ccmr2_write(tim3_ccmr2_read() & value),
        _ => (),
    }


}
pub fn tim3_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccer_write(tim3_ccer_read() | value),
        31 => tim3_ccer_write(tim3_ccer_read() & value),
        _ => (),
    }


}
pub fn tim3_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_cnt_write(tim3_cnt_read() | value),
        31 => tim3_cnt_write(tim3_cnt_read() & value),
        _ => (),
    }


}
pub fn tim3_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_psc_write(tim3_psc_read() | value),
        31 => tim3_psc_write(tim3_psc_read() & value),
        _ => (),
    }


}
pub fn tim3_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_arr_write(tim3_arr_read() | value),
        31 => tim3_arr_write(tim3_arr_read() & value),
        _ => (),
    }


}
pub fn tim3_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccr1_write(tim3_ccr1_read() | value),
        31 => tim3_ccr1_write(tim3_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim3_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccr2_write(tim3_ccr2_read() | value),
        31 => tim3_ccr2_write(tim3_ccr2_read() & value),
        _ => (),
    }


}
pub fn tim3_ccr3_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccr3_write(tim3_ccr3_read() | value),
        31 => tim3_ccr3_write(tim3_ccr3_read() & value),
        _ => (),
    }


}
pub fn tim3_ccr4_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_ccr4_write(tim3_ccr4_read() | value),
        31 => tim3_ccr4_write(tim3_ccr4_read() & value),
        _ => (),
    }


}
pub fn tim3_dcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_dcr_write(tim3_dcr_read() | value),
        31 => tim3_dcr_write(tim3_dcr_read() & value),
        _ => (),
    }


}
pub fn tim3_dmar_seti(value: u32) {
    match value.count_ones() {
        1 => tim3_dmar_write(tim3_dmar_read() | value),
        31 => tim3_dmar_write(tim3_dmar_read() & value),
        _ => (),
    }


}
    
        
pub fn tim4_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_cr1_write(tim4_cr1_read() | value),
        31 => tim4_cr1_write(tim4_cr1_read() & value),
        _ => (),
    }


}
pub fn tim4_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_cr2_write(tim4_cr2_read() | value),
        31 => tim4_cr2_write(tim4_cr2_read() & value),
        _ => (),
    }


}
pub fn tim4_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_smcr_write(tim4_smcr_read() | value),
        31 => tim4_smcr_write(tim4_smcr_read() & value),
        _ => (),
    }


}
pub fn tim4_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_dier_write(tim4_dier_read() | value),
        31 => tim4_dier_write(tim4_dier_read() & value),
        _ => (),
    }


}
pub fn tim4_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_sr_write(tim4_sr_read() | value),
        31 => tim4_sr_write(tim4_sr_read() & value),
        _ => (),
    }


}

pub fn tim4_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccmr1_write(tim4_ccmr1_read() | value),
        31 => tim4_ccmr1_write(tim4_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim4_ccmr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccmr2_write(tim4_ccmr2_read() | value),
        31 => tim4_ccmr2_write(tim4_ccmr2_read() & value),
        _ => (),
    }


}
pub fn tim4_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccer_write(tim4_ccer_read() | value),
        31 => tim4_ccer_write(tim4_ccer_read() & value),
        _ => (),
    }


}
pub fn tim4_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_cnt_write(tim4_cnt_read() | value),
        31 => tim4_cnt_write(tim4_cnt_read() & value),
        _ => (),
    }


}
pub fn tim4_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_psc_write(tim4_psc_read() | value),
        31 => tim4_psc_write(tim4_psc_read() & value),
        _ => (),
    }


}
pub fn tim4_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_arr_write(tim4_arr_read() | value),
        31 => tim4_arr_write(tim4_arr_read() & value),
        _ => (),
    }


}
pub fn tim4_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccr1_write(tim4_ccr1_read() | value),
        31 => tim4_ccr1_write(tim4_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim4_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccr2_write(tim4_ccr2_read() | value),
        31 => tim4_ccr2_write(tim4_ccr2_read() & value),
        _ => (),
    }


}
pub fn tim4_ccr3_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccr3_write(tim4_ccr3_read() | value),
        31 => tim4_ccr3_write(tim4_ccr3_read() & value),
        _ => (),
    }


}
pub fn tim4_ccr4_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_ccr4_write(tim4_ccr4_read() | value),
        31 => tim4_ccr4_write(tim4_ccr4_read() & value),
        _ => (),
    }


}
pub fn tim4_dcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_dcr_write(tim4_dcr_read() | value),
        31 => tim4_dcr_write(tim4_dcr_read() & value),
        _ => (),
    }


}
pub fn tim4_dmar_seti(value: u32) {
    match value.count_ones() {
        1 => tim4_dmar_write(tim4_dmar_read() | value),
        31 => tim4_dmar_write(tim4_dmar_read() & value),
        _ => (),
    }


}
    
        
pub fn tim5_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_cr1_write(tim5_cr1_read() | value),
        31 => tim5_cr1_write(tim5_cr1_read() & value),
        _ => (),
    }


}
pub fn tim5_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_cr2_write(tim5_cr2_read() | value),
        31 => tim5_cr2_write(tim5_cr2_read() & value),
        _ => (),
    }


}
pub fn tim5_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_smcr_write(tim5_smcr_read() | value),
        31 => tim5_smcr_write(tim5_smcr_read() & value),
        _ => (),
    }


}
pub fn tim5_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_dier_write(tim5_dier_read() | value),
        31 => tim5_dier_write(tim5_dier_read() & value),
        _ => (),
    }


}
pub fn tim5_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_sr_write(tim5_sr_read() | value),
        31 => tim5_sr_write(tim5_sr_read() & value),
        _ => (),
    }


}

pub fn tim5_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccmr1_write(tim5_ccmr1_read() | value),
        31 => tim5_ccmr1_write(tim5_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim5_ccmr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccmr2_write(tim5_ccmr2_read() | value),
        31 => tim5_ccmr2_write(tim5_ccmr2_read() & value),
        _ => (),
    }


}
pub fn tim5_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccer_write(tim5_ccer_read() | value),
        31 => tim5_ccer_write(tim5_ccer_read() & value),
        _ => (),
    }


}
pub fn tim5_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_cnt_write(tim5_cnt_read() | value),
        31 => tim5_cnt_write(tim5_cnt_read() & value),
        _ => (),
    }


}
pub fn tim5_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_psc_write(tim5_psc_read() | value),
        31 => tim5_psc_write(tim5_psc_read() & value),
        _ => (),
    }


}
pub fn tim5_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_arr_write(tim5_arr_read() | value),
        31 => tim5_arr_write(tim5_arr_read() & value),
        _ => (),
    }


}
pub fn tim5_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccr1_write(tim5_ccr1_read() | value),
        31 => tim5_ccr1_write(tim5_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim5_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccr2_write(tim5_ccr2_read() | value),
        31 => tim5_ccr2_write(tim5_ccr2_read() & value),
        _ => (),
    }


}
pub fn tim5_ccr3_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccr3_write(tim5_ccr3_read() | value),
        31 => tim5_ccr3_write(tim5_ccr3_read() & value),
        _ => (),
    }


}
pub fn tim5_ccr4_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_ccr4_write(tim5_ccr4_read() | value),
        31 => tim5_ccr4_write(tim5_ccr4_read() & value),
        _ => (),
    }


}
pub fn tim5_dcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_dcr_write(tim5_dcr_read() | value),
        31 => tim5_dcr_write(tim5_dcr_read() & value),
        _ => (),
    }


}
pub fn tim5_dmar_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_dmar_write(tim5_dmar_read() | value),
        31 => tim5_dmar_write(tim5_dmar_read() & value),
        _ => (),
    }


}
pub fn tim5_or_seti(value: u32) {
    match value.count_ones() {
        1 => tim5_or_write(tim5_or_read() | value),
        31 => tim5_or_write(tim5_or_read() & value),
        _ => (),
    }


}
    
        
pub fn tim9_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_cr1_write(tim9_cr1_read() | value),
        31 => tim9_cr1_write(tim9_cr1_read() & value),
        _ => (),
    }


}
pub fn tim9_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_smcr_write(tim9_smcr_read() | value),
        31 => tim9_smcr_write(tim9_smcr_read() & value),
        _ => (),
    }


}
pub fn tim9_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_dier_write(tim9_dier_read() | value),
        31 => tim9_dier_write(tim9_dier_read() & value),
        _ => (),
    }


}
pub fn tim9_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_sr_write(tim9_sr_read() | value),
        31 => tim9_sr_write(tim9_sr_read() & value),
        _ => (),
    }


}

pub fn tim9_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_ccmr1_write(tim9_ccmr1_read() | value),
        31 => tim9_ccmr1_write(tim9_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim9_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_ccer_write(tim9_ccer_read() | value),
        31 => tim9_ccer_write(tim9_ccer_read() & value),
        _ => (),
    }


}
pub fn tim9_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_cnt_write(tim9_cnt_read() | value),
        31 => tim9_cnt_write(tim9_cnt_read() & value),
        _ => (),
    }


}
pub fn tim9_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_psc_write(tim9_psc_read() | value),
        31 => tim9_psc_write(tim9_psc_read() & value),
        _ => (),
    }


}
pub fn tim9_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_arr_write(tim9_arr_read() | value),
        31 => tim9_arr_write(tim9_arr_read() & value),
        _ => (),
    }


}
pub fn tim9_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_ccr1_write(tim9_ccr1_read() | value),
        31 => tim9_ccr1_write(tim9_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim9_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim9_ccr2_write(tim9_ccr2_read() | value),
        31 => tim9_ccr2_write(tim9_ccr2_read() & value),
        _ => (),
    }


}
    
        
pub fn tim12_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_cr1_write(tim12_cr1_read() | value),
        31 => tim12_cr1_write(tim12_cr1_read() & value),
        _ => (),
    }


}
pub fn tim12_smcr_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_smcr_write(tim12_smcr_read() | value),
        31 => tim12_smcr_write(tim12_smcr_read() & value),
        _ => (),
    }


}
pub fn tim12_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_dier_write(tim12_dier_read() | value),
        31 => tim12_dier_write(tim12_dier_read() & value),
        _ => (),
    }


}
pub fn tim12_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_sr_write(tim12_sr_read() | value),
        31 => tim12_sr_write(tim12_sr_read() & value),
        _ => (),
    }


}

pub fn tim12_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_ccmr1_write(tim12_ccmr1_read() | value),
        31 => tim12_ccmr1_write(tim12_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim12_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_ccer_write(tim12_ccer_read() | value),
        31 => tim12_ccer_write(tim12_ccer_read() & value),
        _ => (),
    }


}
pub fn tim12_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_cnt_write(tim12_cnt_read() | value),
        31 => tim12_cnt_write(tim12_cnt_read() & value),
        _ => (),
    }


}
pub fn tim12_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_psc_write(tim12_psc_read() | value),
        31 => tim12_psc_write(tim12_psc_read() & value),
        _ => (),
    }


}
pub fn tim12_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_arr_write(tim12_arr_read() | value),
        31 => tim12_arr_write(tim12_arr_read() & value),
        _ => (),
    }


}
pub fn tim12_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_ccr1_write(tim12_ccr1_read() | value),
        31 => tim12_ccr1_write(tim12_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim12_ccr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim12_ccr2_write(tim12_ccr2_read() | value),
        31 => tim12_ccr2_write(tim12_ccr2_read() & value),
        _ => (),
    }


}
    
        
pub fn tim10_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_cr1_write(tim10_cr1_read() | value),
        31 => tim10_cr1_write(tim10_cr1_read() & value),
        _ => (),
    }


}
pub fn tim10_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_dier_write(tim10_dier_read() | value),
        31 => tim10_dier_write(tim10_dier_read() & value),
        _ => (),
    }


}
pub fn tim10_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_sr_write(tim10_sr_read() | value),
        31 => tim10_sr_write(tim10_sr_read() & value),
        _ => (),
    }


}

pub fn tim10_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_ccmr1_write(tim10_ccmr1_read() | value),
        31 => tim10_ccmr1_write(tim10_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim10_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_ccer_write(tim10_ccer_read() | value),
        31 => tim10_ccer_write(tim10_ccer_read() & value),
        _ => (),
    }


}
pub fn tim10_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_cnt_write(tim10_cnt_read() | value),
        31 => tim10_cnt_write(tim10_cnt_read() & value),
        _ => (),
    }


}
pub fn tim10_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_psc_write(tim10_psc_read() | value),
        31 => tim10_psc_write(tim10_psc_read() & value),
        _ => (),
    }


}
pub fn tim10_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_arr_write(tim10_arr_read() | value),
        31 => tim10_arr_write(tim10_arr_read() & value),
        _ => (),
    }


}
pub fn tim10_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_ccr1_write(tim10_ccr1_read() | value),
        31 => tim10_ccr1_write(tim10_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim10_or_seti(value: u32) {
    match value.count_ones() {
        1 => tim10_or_write(tim10_or_read() | value),
        31 => tim10_or_write(tim10_or_read() & value),
        _ => (),
    }


}
    
        
pub fn tim11_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_cr1_write(tim11_cr1_read() | value),
        31 => tim11_cr1_write(tim11_cr1_read() & value),
        _ => (),
    }


}
pub fn tim11_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_dier_write(tim11_dier_read() | value),
        31 => tim11_dier_write(tim11_dier_read() & value),
        _ => (),
    }


}
pub fn tim11_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_sr_write(tim11_sr_read() | value),
        31 => tim11_sr_write(tim11_sr_read() & value),
        _ => (),
    }


}

pub fn tim11_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_ccmr1_write(tim11_ccmr1_read() | value),
        31 => tim11_ccmr1_write(tim11_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim11_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_ccer_write(tim11_ccer_read() | value),
        31 => tim11_ccer_write(tim11_ccer_read() & value),
        _ => (),
    }


}
pub fn tim11_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_cnt_write(tim11_cnt_read() | value),
        31 => tim11_cnt_write(tim11_cnt_read() & value),
        _ => (),
    }


}
pub fn tim11_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_psc_write(tim11_psc_read() | value),
        31 => tim11_psc_write(tim11_psc_read() & value),
        _ => (),
    }


}
pub fn tim11_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_arr_write(tim11_arr_read() | value),
        31 => tim11_arr_write(tim11_arr_read() & value),
        _ => (),
    }


}
pub fn tim11_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_ccr1_write(tim11_ccr1_read() | value),
        31 => tim11_ccr1_write(tim11_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim11_or_seti(value: u32) {
    match value.count_ones() {
        1 => tim11_or_write(tim11_or_read() | value),
        31 => tim11_or_write(tim11_or_read() & value),
        _ => (),
    }


}
    
        
pub fn tim13_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_cr1_write(tim13_cr1_read() | value),
        31 => tim13_cr1_write(tim13_cr1_read() & value),
        _ => (),
    }


}
pub fn tim13_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_dier_write(tim13_dier_read() | value),
        31 => tim13_dier_write(tim13_dier_read() & value),
        _ => (),
    }


}
pub fn tim13_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_sr_write(tim13_sr_read() | value),
        31 => tim13_sr_write(tim13_sr_read() & value),
        _ => (),
    }


}

pub fn tim13_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_ccmr1_write(tim13_ccmr1_read() | value),
        31 => tim13_ccmr1_write(tim13_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim13_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_ccer_write(tim13_ccer_read() | value),
        31 => tim13_ccer_write(tim13_ccer_read() & value),
        _ => (),
    }


}
pub fn tim13_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_cnt_write(tim13_cnt_read() | value),
        31 => tim13_cnt_write(tim13_cnt_read() & value),
        _ => (),
    }


}
pub fn tim13_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_psc_write(tim13_psc_read() | value),
        31 => tim13_psc_write(tim13_psc_read() & value),
        _ => (),
    }


}
pub fn tim13_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_arr_write(tim13_arr_read() | value),
        31 => tim13_arr_write(tim13_arr_read() & value),
        _ => (),
    }


}
pub fn tim13_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_ccr1_write(tim13_ccr1_read() | value),
        31 => tim13_ccr1_write(tim13_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim13_or_seti(value: u32) {
    match value.count_ones() {
        1 => tim13_or_write(tim13_or_read() | value),
        31 => tim13_or_write(tim13_or_read() & value),
        _ => (),
    }


}
    
        
pub fn tim14_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_cr1_write(tim14_cr1_read() | value),
        31 => tim14_cr1_write(tim14_cr1_read() & value),
        _ => (),
    }


}
pub fn tim14_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_dier_write(tim14_dier_read() | value),
        31 => tim14_dier_write(tim14_dier_read() & value),
        _ => (),
    }


}
pub fn tim14_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_sr_write(tim14_sr_read() | value),
        31 => tim14_sr_write(tim14_sr_read() & value),
        _ => (),
    }


}

pub fn tim14_ccmr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_ccmr1_write(tim14_ccmr1_read() | value),
        31 => tim14_ccmr1_write(tim14_ccmr1_read() & value),
        _ => (),
    }


}
pub fn tim14_ccer_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_ccer_write(tim14_ccer_read() | value),
        31 => tim14_ccer_write(tim14_ccer_read() & value),
        _ => (),
    }


}
pub fn tim14_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_cnt_write(tim14_cnt_read() | value),
        31 => tim14_cnt_write(tim14_cnt_read() & value),
        _ => (),
    }


}
pub fn tim14_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_psc_write(tim14_psc_read() | value),
        31 => tim14_psc_write(tim14_psc_read() & value),
        _ => (),
    }


}
pub fn tim14_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_arr_write(tim14_arr_read() | value),
        31 => tim14_arr_write(tim14_arr_read() & value),
        _ => (),
    }


}
pub fn tim14_ccr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_ccr1_write(tim14_ccr1_read() | value),
        31 => tim14_ccr1_write(tim14_ccr1_read() & value),
        _ => (),
    }


}
pub fn tim14_or_seti(value: u32) {
    match value.count_ones() {
        1 => tim14_or_write(tim14_or_read() | value),
        31 => tim14_or_write(tim14_or_read() & value),
        _ => (),
    }


}
    
        
pub fn tim6_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_cr1_write(tim6_cr1_read() | value),
        31 => tim6_cr1_write(tim6_cr1_read() & value),
        _ => (),
    }


}
pub fn tim6_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_cr2_write(tim6_cr2_read() | value),
        31 => tim6_cr2_write(tim6_cr2_read() & value),
        _ => (),
    }


}
pub fn tim6_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_dier_write(tim6_dier_read() | value),
        31 => tim6_dier_write(tim6_dier_read() & value),
        _ => (),
    }


}
pub fn tim6_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_sr_write(tim6_sr_read() | value),
        31 => tim6_sr_write(tim6_sr_read() & value),
        _ => (),
    }


}

pub fn tim6_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_cnt_write(tim6_cnt_read() | value),
        31 => tim6_cnt_write(tim6_cnt_read() & value),
        _ => (),
    }


}
pub fn tim6_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_psc_write(tim6_psc_read() | value),
        31 => tim6_psc_write(tim6_psc_read() & value),
        _ => (),
    }


}
pub fn tim6_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim6_arr_write(tim6_arr_read() | value),
        31 => tim6_arr_write(tim6_arr_read() & value),
        _ => (),
    }


}
    
        
pub fn tim7_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_cr1_write(tim7_cr1_read() | value),
        31 => tim7_cr1_write(tim7_cr1_read() & value),
        _ => (),
    }


}
pub fn tim7_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_cr2_write(tim7_cr2_read() | value),
        31 => tim7_cr2_write(tim7_cr2_read() & value),
        _ => (),
    }


}
pub fn tim7_dier_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_dier_write(tim7_dier_read() | value),
        31 => tim7_dier_write(tim7_dier_read() & value),
        _ => (),
    }


}
pub fn tim7_sr_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_sr_write(tim7_sr_read() | value),
        31 => tim7_sr_write(tim7_sr_read() & value),
        _ => (),
    }


}

pub fn tim7_cnt_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_cnt_write(tim7_cnt_read() | value),
        31 => tim7_cnt_write(tim7_cnt_read() & value),
        _ => (),
    }


}
pub fn tim7_psc_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_psc_write(tim7_psc_read() | value),
        31 => tim7_psc_write(tim7_psc_read() & value),
        _ => (),
    }


}
pub fn tim7_arr_seti(value: u32) {
    match value.count_ones() {
        1 => tim7_arr_write(tim7_arr_read() | value),
        31 => tim7_arr_write(tim7_arr_read() & value),
        _ => (),
    }


}
    