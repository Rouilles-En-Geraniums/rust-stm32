# Rust -> STM32

Providing a way to compile any Rust program for STM32

## Installation

Install rust by following the instructions at [rustc] (https://rustup.rs)

In a bare metal environement, we can not load the standard library. To prevent rust from loading the library use **no_std**. 

Or you can install cargo no-std-check which ensure thet your library does not link to **libstd** :
 
> cargo no-std-check

Then run this command on a crate to build it's lib target without access to **std** :

> cargo no-std-check --manifest-path nostd/cargo.toml

## Installing tools

Make sure you have a compiler version equal or newer than 1.31.

>rustc -V 

To add cross-compilation for ARM cortex-M arcitectures choose one of the following targets : 

Cortex-M0, M0+, and M1 : 
>rustup target add thumbv6m-none-eabi

Cortex-M3 : 
>rustup target add thumbv7m-none-eabi

Cortex-M4 and M7 : 
>rustup target add thumbv7em-none-eabi

Cortex-M4F and M7F :
>rustup target add thumbv7em-none-eabihf

Cortex-M23 :
>rustup target add thumbv8m.base-none-eabi

Cortex-M33 and M35P :
>rustup target add thumbv8m.main-none-eabi

Cortex-M33F and M35PF
>rustup target add thumbv8m.main-none-eabihf

### Cargo-binutils

>cargo install cargo-binutils 

>rustup component add llvm-tools-preview

### Cargo-generate

>cargo install cargo-generate

### OS-Specific Instrucions

#### Linux 
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

>git clone https://

## Cross-compiling

The next step is to cross compile the program for the desired target. Figure out what target architecture you are working with, the .cargo/config.toml contains multiple target that you can compile too, you just need to uncomment the right one.

```
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```

The target is not automatically installed with the Rust toolchain, make sure you add it before cross compiling.

Since the compilation target has been set as default in your .cargo/config.toml 

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
You can now cross compile programs using **cargo build** 

>cargo build --release

The output binary will be located at target/thumbv7em-none-eabi/release/project-name, this file will be loaded on the chip next.


# Debugging 

Make sure to change you **openocd.cfg** file to connect the ST-LINK on the board. And change the project-file-name in the Makefile. 


Run this command from the root : 

>make openocd  

And on another terminal in the root as well :

>make debug

Your program is now loaded. The semihosting is enabled if you are planning on communication and using the I/O of the host computer.



