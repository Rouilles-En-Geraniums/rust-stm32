#![no_std]

use core::panic::PanicInfo;
use core::ptr;

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


    let count = &_data_begin as *const u8 as usize - &_data_end as *const u8 as usize;
    ptr::copy_nonoverlapping(&_data_flash as *const u8, &mut _data_end as *mut u8, count);
    
    let count = &_bss_begin as *const u8 as usize - &_bss_end as *const u8 as usize;
    ptr::write_bytes(&mut _bss_end as *mut u8, 0, count);


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
    loop {}
}
