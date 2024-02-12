TARGET =
OPENOCD_CFG = openocd.cfg 
GDB =
APP =
OBJDMP = arm-none-eabi-objdump

openocd :
	openocd -f $(OPENOCD_CFG) 	

debug :
	cargo clean
	cargo build

objdump :
	cargo clean
	cargo build
	$(OBJDMP) -h target/$(TARGET)/debug/$(APP)

flashdebug :
	cargo clean
	cargo build
	$(GDB) -q target/$(TARGET)/debug/$(APP) -command gdbinit

flashrelease :
	cargo clean
	cargo build --release 
	$(GDB) -q target/$(TARGET)/release/$(APP) -ex "target remote :3333" -ex "load"


