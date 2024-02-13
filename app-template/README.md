
[![Licence](https://img.shields.io/badge/licence-AGPL--3.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Last-changedate](https://img.shields.io/badge/last%20change-12--02--2024-yellowgreen.svg)](https://github.com/SMAC-Group/gmwm)

# STM Microcontroller Cross-Compilation for Rust Projects

This project offers a method for cross-compiling Rust programs specifically for STM microcontrollers.

## Installation

Install rust by following the instructions at [rustc](https://rustup.rs)

In a bare metal environement, we can not load the standard library. To prevent rust from loading the library use **no_std**. 

Or you can install cargo no-std-check which ensure thet your library does not link to **libstd** :
 
> cargo no-std-check

Then run this command on a crate to build it's lib target without access to **std** :

> cargo no-std-check --manifest-path nostd/cargo.toml

## Installing tools

Make sure you have a compiler version equal or newer than 1.31.

>rustc -V 

To add cross-compilation for ARM cortex-M arcitectures choose your target : 

Cortex-M4 and M7 : 
>rustup target add thumbv7em-none-eabi

Install your target, and if it differs from the one mentioned in the tutorial, search for the appropriate one.

### Cargo-binutils

Cargo-binutils is a collection of Cargo subcommands that make it easy to use the LLVM tools that are shipped with the Rust toolchain

>cargo install cargo-binutils 

>rustup component add llvm-tools-preview

### OS-Specific Instrucions

##### Ubuntu 
>sudo apt-get install openocd

>sudo apt-get install gdb-architecture 

Or 
>sudo apt-get install gdb

##### Fedora
>sudo dnf install openocd

>sudo dnf install gdb-architecture 

Or 
>sudo dnf install gdb

## udev rules 
This rule lets you use OpenOCD with the Discovery board without root privilege.

Create the file /etc/udev/rules.d/70-st-link.rules with the contents shown below.

```
# STM32F3DISCOVERY rev A/B - ST-LINK/V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", TAG+="uaccess"
# STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", TAG+="uaccess" 
```


#### MacOs
>brew install armmbed/formulae/arm-none-eabi-gcc

>brew install openocd


# Getting started 

## Clone the repository 

>git clone git@github.com:Rouilles-En-Geraniums/rust-stm32.git

## Cross-compiling

The next step is to cross compile the program for the desired target. Figure out what target architecture you are working with as said before, the .cargo/config.toml contains multiple target that you can compile too, you just need to write the right one.

```
[build]
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```

The target is not automatically installed with the Rust toolchain, make sure you add it before cross compiling.

Since the compilation target has been set as default in your .cargo/config.toml 

### Linker script

The next step is to modify the memory region information in the memory.x file : 

```
/* Linker script for the STM32F407 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
}
```
### openocd.cfg

Modify the file to match the microcontroller you are currently using. To identify the supported target for OpenOCD, refer to the following link. [target](https://github.com/analogdevicesinc/openocd/tree/master/tcl/target)

## Final steps... 
Import geranium_rt


### main.rs
explain plus make sure handle_TIM is always respected


You can now cross compile programs using **cargo build** 

>cargo build --release   #mode not debug 

>cargo build 

The output binary will be located at target/thumbv7em-none-eabi/release/project-name, this file will be loaded on the chip next.


# Debugging 

Make sure to change you **openocd.cfg** file to connect the ST-LINK on the board. And change the project-file-name in the Makefile. 


Run this command from the root : 

>make openocd  

And on another terminal in the root as well :

>make debug

Your program is now loaded. The semihosting is enabled if you are planning on communication and using the I/O of the host computer.



