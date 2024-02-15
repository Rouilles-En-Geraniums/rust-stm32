
[![Licence](https://img.shields.io/badge/licence-AGPL--3.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Last-changedate](https://img.shields.io/badge/last%20change-12--02--2024-yellowgreen.svg)](https://github.com/SMAC-Group/gmwm)

# STM Microcontroller Cross-Compilation for Rust Projects

This project offers a method for cross-compiling Rust programs specifically for STM microcontrollers.

## Installation

This library requires Rust and Python along with various other tools to work.

It is recommended to first install Rustup as it will make the installation of other packages easier. You can find more information at https://www.rust-lang.org/tools/install.

This project requires Rustc 1.7 or above.

It is assumed that you have Python already installed on your machine. This project requires Python 3.9 or above.

### Cross-compilation tools

#### Target

You need to add the cross-compilation rust compiler for the ARM cortex-M arcitecture that your target uses :

STM32F407-G DISC1 have a Cortex-M4 which requires the thumbv7em-none-eabi target. It can be installed with :
>rustup target add thumbv7em-none-eabi

Other targets may be :
```
thumbv6m-none-eabi    # Cortex-M0 and Cortex-M0+
thumbv7m-none-eabi    # Cortex-M3
thumbv7em-none-eabi   # Cortex-M4 and Cortex-M7 (no FPU)
thumbv7em-none-eabihf # Cortex-M4F and Cortex-M7F (with FPU)
```

If you cannot find your target, feel free to look it up online.

#### Cargo-binutils

Cargo-binutils is a collection of Cargo subcommands that make it easy to use the LLVM tools that are shipped with the Rust toolchain. They are required for this project.

> cargo install cargo-binutils

> rustup component add llvm-tools-preview


### OpenOCD

OpenOCD is the tool used to communicate with the board. 

#### Ubuntu

> sudo apt-get install openocd

> sudo apt-get install gdb-architecture

Or
> sudo apt-get install gdb

##### Fedora
> sudo dnf install openocd

> sudo dnf install gdb-architecture

Or
> sudo dnf install gdb

#### Windows 

##### Openocd 

- Download the openocd toolchain from : https://gnutoolchains.com/arm-eabi/openocd/

  Extract the folder, place it where you want it to be installed, and add its installation location to your PATH.

##### STLink Driver

- Download the STM32 STLink Driver Software and install it : https://www.st.com/en/development-tools/stsw-link009.html#get-software

  Downloading this driver requires submitting your email address and waiting until you receieve the email (may take up to 5mins). Extract and follow the instructions once downloaded.

  This Driver is necessary for openocd to access your STM32 Board through USB, as it will not be recognized otherwise.

##### GDB Multiarch (MSYS2)

- Download and install gdb-multiarch. We recommend using MSYS2 for this : https://www.msys2.org/

  Once msys2 is install, install the gdb-multiarch package : 
  > pacman -S mingw-w64-x86_64-gdb-multiarch

  This should automatically add gdb-multiarch to your path. More information at https://packages.msys2.org/package/mingw-w64-x86_64-gdb-multiarch?repo=mingw64

Adding anything to your PATH may require a reboot.


#### MacOs

It is recommended to install packages with homebrew.

##### Openocd 

- Install the openocd package with brew :
  > brew install openocd

##### Arm-none-eabi-gdb

- install the arm-none-eabi-gdb package with brew :
  > brew install arm-none-eabi-gdb


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



