TARGET = thumbv7em-none-eabi
OPENOCD_CFG = openocd.cfg 

openocd :
	openocd -f $(OPENOCD_CFG) 

debug : 
	cargo build --release 
	gdb -ex "target remote :3333" -ex "monitor arm semihosting enable" -ex "load" target/thumbv7em-none-eabi/release/cross-compilation