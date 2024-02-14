extern crate core;
use core::format_args;
use core::fmt::Write;
use core::fmt::write;
use core::fmt::Arguments;

use crate::stm32rustlib::various::*;
use crate::stm32rustlib::gdb::*;


pub fn print(args: Arguments<'_>) {
    let mut p = Printer;
    p.print(args);
}

pub fn println(args: Arguments<'_>) {
    let mut p = Printer;
    p.println(args);
}

struct Printer;

impl Printer {
    fn print(&mut self, args: Arguments<'_>) -> () {
        let _ = write(self, args); //
    }

    fn println(&mut self, args: Arguments<'_>) -> () {
        let _ = write(self, args);
        let _ = self.write_char('\n');
    }
}

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for ch in s.chars() {
            while (itm_stimulus_port0_read() & 1) == 0 {}
            itm_stimulus_port0_write(ch as u32);
        }
        Ok(())
    }
}
