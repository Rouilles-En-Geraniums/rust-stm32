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

>```# STM32F3DISCOVERY rev A/B - ST-LINK/V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", TAG+="uaccess"
# STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", TAG+="uaccess"```

#### MacOs
>brew install armmbed/formulae/arm-none-eabi-gcc

>brew install openocd

