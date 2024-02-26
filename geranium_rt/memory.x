/* Memory layout of the STM32f407G-DISC1 microcontroller */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
}

/* The entry point is the reset handler */
ENTRY(HandlerReset)

EXTERN(RESET_VECTOR);
EXTERN(EXCEPTIONS);
EXTERN(CUSTOM_EXCEPTIONS);

PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultSystickHandler);

SECTIONS
{
	.vector_table ORIGIN(FLASH) :
	{
		_vectab_begin = .; /* begin of vector table in flash (=0) */
		/* First entry: initial Stack Pointer value */
		LONG(ORIGIN(RAM) + LENGTH(RAM));

		/* Second entry: reset vector */
		KEEP(*(.vector_table.reset_vector));

		/* The next 14 entries are exception vectors */
		KEEP(*(.vector_table.exceptions));

		/* The next ???? entries are custom exception vectors */
		KEEP(*(.vector_table.custom_exceptions));
		_vectab_end = .; /* end of vector table in flash (=size of vectab) */
	} > FLASH

	.text :
	{
		*(.text)
		*(.text*)
		. = ALIGN(4);
	} > FLASH

	.rodata :
	{
		*(.rodata)
		*(.rodata*)
		. = ALIGN(4);
	} > FLASH
	_data_flash = .;



	.data : AT(_data_flash)
	{
		_vectab_in_ram = .; /* begin of vector table to copy in RAM */
		. += 0x400; /* . += (vectab_end - vectab_begin) ??*/
		_data_begin = .; /* begin of data to copy in RAM */
		*(.init)
		*(.init*)
		*(.fini)
		*(.fini*)
		*(.data)
		*(.data*)
		. = ALIGN(4);
		_data_end = .;
	} > RAM

	/* Un-initialized static variables */
	.bss :
	{
		_bss_begin = .;
		*(.bss)
		*(.bss*)
		. = ALIGN(4);
		_bss_end = .;
	} > RAM

	_stack_size = 1024;
	_stack_end = ORIGIN(RAM) + LENGTH(RAM);
	_stack_begin = _stack_end - _stack_size;
	. = _stack_begin;
	.stack :
	{
		. = . + _stack_size;
	} > RAM

	
	/DISCARD/ :
	{
		*(.ARM.exidx .ARM.exidx.*);
	}
}
