#![no_std]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use core::ptr;
use cortex_m_semihosting::hprintln;

extern crate core;
pub mod stm32rustlib;
use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::various::*;


pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}


extern "C" {
    static mut _data_flash: u8;
    static mut _data_begin: u8;
    static mut _data_end: u8;
    static mut _bss_begin: u8;
    static mut _bss_end: u8;
    static mut _stack_end: u8;
}


#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: DefaultExceptionHandler },  // 2 - NMI (non maskable interrupt)
    Vector { handler: DefaultExceptionHandler }, // 3 - Hard fault
    Vector { handler: DefaultExceptionHandler }, // 4 - Memmanage fault (MPU violation or access to illegal location)
    Vector { handler: DefaultExceptionHandler }, // 5 - Bus fault (bus error)
    Vector { handler: DefaultExceptionHandler }, // 6 - Usage fault (program error eg acces coprocessor)
    Vector { reserved: 0 }, // 7 - Reserved
    Vector { reserved: 0 }, // 8 - Reserved
    Vector { reserved: 0 }, // 9 - Reserved
    Vector { reserved: 0 }, // 10 - Reserved
    Vector { handler: DefaultExceptionHandler }, // 11 - SVC (Supervisor call)
    Vector { reserved: 0 }, // 12 - Debug monitor (BP, WP, external debug requests)
    Vector { reserved: 0 }, // 13 - Reserved
    Vector { handler: DefaultExceptionHandler }, // 14 - PendSV (pendable service call)
    Vector { handler: DefaultExceptionHandler }, // 15 - SysTick (System tick timer)
];


#[link_section = ".vector_table.custom_exceptions"]
#[no_mangle]
pub static CUSTOM_EXCEPTIONS: [Vector; 44] = [
    Vector { handler: DefaultExceptionHandler }, // 16 
    Vector { handler: DefaultExceptionHandler }, // 17
    Vector { handler: DefaultExceptionHandler }, // 18
    Vector { handler: DefaultExceptionHandler }, // 19
    Vector { handler: DefaultExceptionHandler }, // 21
    Vector { handler: DefaultExceptionHandler }, // 22
    Vector { handler: DefaultExceptionHandler }, // 23
    Vector { handler: DefaultExceptionHandler }, // 24
    Vector { handler: DefaultExceptionHandler }, // 25
    Vector { handler: DefaultExceptionHandler }, // 26
    Vector { handler: DefaultExceptionHandler }, // 27
    Vector { handler: DefaultExceptionHandler }, // 28
    Vector { handler: DefaultExceptionHandler }, // 29
    Vector { handler: DefaultExceptionHandler }, // 30
    Vector { handler: DefaultExceptionHandler }, // 31
    Vector { handler: DefaultExceptionHandler }, // 32
    Vector { handler: DefaultExceptionHandler }, // 33
    Vector { handler: DefaultExceptionHandler }, // 34
    Vector { handler: DefaultExceptionHandler }, // 35
    Vector { handler: DefaultExceptionHandler }, // 36
    Vector { handler: DefaultExceptionHandler }, // 37
    Vector { handler: DefaultExceptionHandler }, // 38
    Vector { handler: DefaultExceptionHandler }, // 39
    Vector { handler: DefaultExceptionHandler }, // 40
    Vector { handler: DefaultExceptionHandler }, // 41
    Vector { handler: DefaultExceptionHandler }, // 42
    Vector { handler: DefaultExceptionHandler }, // 43
    Vector { handler: DefaultExceptionHandler }, // 44
    Vector { handler: DefaultExceptionHandler }, // 45
    Vector { handler: DefaultExceptionHandler }, // 46
    Vector { handler: DefaultExceptionHandler }, // 47
    Vector { handler: DefaultExceptionHandler }, // 48
    Vector { handler: DefaultExceptionHandler }, // 49
    Vector { handler: DefaultExceptionHandler }, // 50
    Vector { handler: DefaultExceptionHandler }, // 51
    Vector { handler: DefaultExceptionHandler }, // 52
    Vector { handler: DefaultExceptionHandler }, // 53
    Vector { handler: DefaultExceptionHandler }, // 54
    Vector { handler: DefaultExceptionHandler }, // 55
    Vector { handler: DefaultExceptionHandler }, // 56
    Vector { handler: DefaultExceptionHandler }, // 57
    Vector { handler: DefaultExceptionHandler }, // 58
    Vector { handler: DefaultExceptionHandler }, // 59
    Vector { handler: DefaultExceptionHandler }, // 60
];


#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = HandlerReset;


// Reset Handler  first (and only) thing called when rebooting or starting up
#[no_mangle]
pub unsafe extern "C" fn HandlerReset() -> ! {    
	// configure clock
	// HSI clock = 16 MHz
	// HSE_VALUE <- 8 000 000 (8MHz - crystal frequency)
	// PLL_M <- 8 (/8 - 1MHz PLL)
	// MCK = (HS[EI]_CK / PLL_M) * PLL_N / PLL_P
	//	MCK		APB1	APB2	PLL_M	PLL_N	PLL_P	PLL_Q
	//	168		42		84		8		336		2		7
    
	// configure HSI clock
	//RCC_CR |= RCC_CR_HSION;
    rcc_cr_write(rcc_cr_read() | RCC_CR_HSION);
	//while((RCC_CR & RCC_CR_HSIRDY) == 0);
    while (rcc_cr_read() & RCC_CR_HSIRDY) == 0 {}
	//RCC_CFGR_SW_SET(RCC_CFGR, RCC_HSI);
    rcc_cfgr_write(rep_bits(rcc_cfgr_read(),0,2,0b00));
    
    
	// configure HSE clock
	//RCC_CR |= RCC_CR_HSEON;
    rcc_cr_write(rcc_cr_read() | RCC_CR_HSEON);
	//while((RCC_CR & RCC_CR_HSERDY) == 0);
    while (rcc_cr_read() & RCC_CR_HSERDY) == 0 {}
    
	// configure AHB and AHP[12]
	//RCC_CFGGR_HPRE_SET(RCC_CFGR, RCC_HPRE_NODIV);
    rcc_cfgr_write(rep_bits(rcc_cfgr_read(),4,4,0b0000));
	//RCC_CFGGR_PPRE1_SET(RCC_CFGR, RCC_PPRE_DIV4);
    rcc_cfgr_write(rep_bits(rcc_cfgr_read(),10,3,0b110));
	//RCC_CFGGR_PPRE2_SET(RCC_CFGR, RCC_PPRE_DIV2);
    rcc_cfgr_write(rep_bits(rcc_cfgr_read(),13,3,0b100));
    
	// configure PLL
	//RCC_CR &= ~RCC_CR_PLLON;
    rcc_cr_write(rcc_cr_read() & !RCC_CR_PLLON);
    let mut x : u32 = 0;
	//RCC_PLLCFGR_M_SET(x, 8);
    x = rep_bits(x,0,5,8);
	//RCC_PLLCFGR_N_SET(x, 336);
    x = rep_bits(x,6,9,336);
	//RCC_PLLCFGR_P_SET(x, RCC_PLLP2);
    x = rep_bits(x,16,2,0);
	//x |= RCC_PLLCFGR_SRC_HSE;
    x |= RCC_PLLCFGR_PLLSRC;
	//RCC_PLLCFGR_Q_SET(x, 7);
    x = rep_bits(x,24,4,7);
	//RCC_PLLCFGR = x;
    rcc_pllcfgr_write(x);
	//RCC_CR |= RCC_CR_PLLON;
    rcc_cr_write(rcc_cr_read() | RCC_CR_PLLON);
	//while((RCC_CR & RCC_CR_PLLRDY) == 0);
    while (rcc_cr_read() & RCC_CR_PLLRDY) == 0 {}
    
	// configure flash
	x = flash_acr_read();
	x |= FLASH_ACR_DCEN;
	x |= FLASH_ACR_ICEN;
	//FLASH_ACR_LATENCY_SET(x, 5);
    x = rep_bits(x,FLASH_ACR_LATENCY,2,5);
	//FLASH_ACR = x;
    flash_acr_write(x);
	
	// select PLL as SYSCLK
	//RCC_CFGR_SW_SET(RCC_CFGR, RCC_PLL);
    rcc_cfgr_write(rep_bits(rcc_cfgr_read(),0,2,0b10));
    //while(RCC_CFGR_SWS_GET(RCC_CFGR) != RCC_PLL);
    while get_bits(rcc_cfgr_read(),2,2) != 0b10 {}   
	//RCC_CR &= ~RCC_CR_HSION;
    rcc_cr_write(rcc_cr_read() & !RCC_CR_HSION);

	// rmap SRAM at 0
	//SYSCFG_MEMRMP = 0b11;
    syscfg_memrmp_write(0b11);

    //uint32_t *p;
	//uint32_t *q;
    
	// copy data from FLASH to RAM
	//p = &_data_flash;
	//for(q = &_data_begin; q < &_data_end;)
	//		*q++ = *p++;
    let count1 = &_data_end as *const u8 as usize - &_data_begin as *const u8 as usize;
    ptr::copy_nonoverlapping(&_data_flash as *const u8, &mut _data_begin as *mut u8, count1);
    
	// set to 0 BSS
	//for(q = &_bss_begin; q < &_bss_end;)
	//	*q++ = 0;
    let count2 = &_bss_end as *const u8 as usize - &_bss_begin as *const u8 as usize;
    //let count2 = 1024;
    ptr::write_bytes(&mut _bss_begin as *mut u8, 0, count2);


	// console configuration
	//DBG_DEMCR |= DBG_DEMCR_TRCENA;
    dbg_demcr_write(dbg_demcr_read() | DBG_DEMCR_TRCENA);
	//ITM_TRACE_EN |= ITM_TRACE_EN_PORT0;
    itm_trace_en_write(itm_trace_en_read() | ITM_TRACE_EN_PORT0);


	extern "Rust" {
        fn main() -> !;
    }
    main()
}


// default handler if something goes wrong
#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    hprintln!("panic info : {}", _panic).unwrap();
    loop {}
}
