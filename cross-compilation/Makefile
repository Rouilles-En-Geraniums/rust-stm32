TARGET = thumbv7em-none-eabi
OPENOCD_CFG = openocd.cfg 
GDB = gdb
#GDB = arm-none-eabi-gdb

openocd :
	openocd -f $(OPENOCD_CFG) 	

debug :
	cargo clean
	cargo build

objdump :
	cargo clean
	cargo build
	arm-none-eabi-objdump -h target/$(TARGET)/debug/cross-compilation

flashdebug :
	cargo clean
	cargo build
	$(GDB) -q target/$(TARGET)/debug/cross-compilation -ex "target remote :3333" -ex "load"

flashrelease :
	cargo clean
	cargo build --release 
	$(GDB) -q target/$(TARGET)/release/cross-compilation -ex "target remote :3333" -ex "load"


