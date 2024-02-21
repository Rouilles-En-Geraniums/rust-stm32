
[![Licence](https://img.shields.io/badge/licence-AGPL--3.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Last-changedate](https://img.shields.io/badge/last%20change-15--02--2024-yellowgreen.svg)](https://github.com/SMAC-Group/gmwm)

# STM Microcontroller Cross-Compilation for Rust Projects

This project offers a method for cross-compiling Rust programs specifically for STM microcontrollers.

This project was done as part of a university module at Université Toulouse III - Paul Sabatier: SECIL Master's Degree.

It has been tested for the following operating systems :
- Linux:
    - Fedora 38;
    - Ubuntu 22.04LTS;
- Windows:
    - 10;
    - 11;
    - WSL;
- MacOS:
    - Ventura 13.0.1 (M1 Chip).

## Installation

This library requires Rust and Python along with various other tools to work.

It is recommended to first install Rustup as it will make the installation of other packages easier. You can find more information at https://www.rust-lang.org/tools/install.

This project requires Rustc 1.7 or above.

It is assumed that you have Python already installed on your machine. This project requires Python 3.9 or above.

### Cross-compilation tools

#### Target

You need to add the cross-compilation rust compiler for the ARM cortex-M arcitecture that your target uses :

STM32F407-G DISC1 have a Cortex-M4 which requires the thumbv7em-none-eabi target. It can be installed with :
> `rustup target add thumbv7em-none-eabi`

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

> `cargo install cargo-binutils`

> `rustup component add llvm-tools-preview`


### OpenOCD

OpenOCD is the tool used to communicate with the board. 

#### Ubuntu

> `sudo apt-get install openocd`

> `sudo apt-get install gdb-architecture`

Or
> `sudo apt-get install gdb`

##### Fedora
> `sudo dnf install openocd`

> `sudo dnf install gdb-architecture`

Or
> `sudo dnf install gdb`

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
  > `pacman -S mingw-w64-x86_64-gdb-multiarch`

  This should automatically add gdb-multiarch to your path. More information at https://packages.msys2.org/package/mingw-w64-x86_64-gdb-multiarch?repo=mingw64

Adding anything to your PATH may require a reboot.


#### MacOs

It is recommended to install packages with homebrew.

##### Openocd 

- Install the openocd package with brew :
  > `brew install openocd`

##### Arm-none-eabi-gdb

- install the arm-none-eabi-gdb package with brew :
  > `brew install arm-none-eabi-gdb`


# Usage

## Clone the repository

Start of by cloning the repository :
> `git clone git@github.com:Rouilles-En-Geraniums/rust-stm32.git`

## Generate a new project

New projects can be generated using the `gen-new-project.py` script. See `-h` for options.

By default, this script generates new projects based on the `app-template/` folder. Feel free to edit this script at your convenience.

The default target of this project is the STM32F407-G DISC1 Board. If you wish to build for an other target, several changes have to be done. These changes have to be made within the `geranium_rt/` library crate as well as the `app-template/` binary crate, which contain everything that is required for the project to build :
- `geranium_rt/` :
    - `memory.x` : the Linker Script ;
    - `Cargo.toml` : the toml file with the minimal imports for the library crate ;
    - `build.rs` : which tells cargo to use the Linker Script ;
    - `src/lib.rs` : which is what will first be executed when loading the compiled program on the board, see below for more details ;
    - `src/stm32rustlib/` : the generated abstraction library itself, which projects will directly include 
- `app-template/` :
    - `Cargo.toml` : the toml file with the minimal imports for the project binary crate ;
    - `gdbinit` : contains various commands executed at the beginning of the debug process (optional) ;
    - `Makefile` : contains various rules to easy the project usage ;
    - `openocd.cfg` : contains configuration rules for openocd to work based on the board ;
    - `.cargo/config.toml` : contains information on the cross-compiling target for cargo to build.

### Cross-compiling

The goal is to cross compile the program for your desired target. Figure out what target architecture you are working with and install the corresponding package via the command in the installation section. By specifying the target to `gen-new-project.py`, the script will generate the adequate `.cargo/config.toml` file and fill in the Makefile properly.

Cargo will automatically change its compiler for the specified target when running cargo build.

### Linker script and lib.rs

Link Scripts are very board-specific and may greatly vary. You may use the exisisting `memory.x` as an example. If you decide to use a different name for your Linker Script, you may need to edit various other files accordingly (automation is not supported).

Once the Linker Script done, you will need to write your own lib.rs file which defines everything that the board requires to work.

STM32 Boards require a Vector Table for interruptions, as well as a Reset Handler that is the first code that is executed on boot or on restart. This function should initialize everything that your projects require such as :
- The Vector Table for interruptions, as well as some handlers (such as the PanicHandler or the DefaultHandler) ;
- Clocks ;
- PLL ;
- Flash ;
- Ram ;
- DBG Console ;
- ...

Feel free to modify the existing lib.rs as you see fit.

### Openocd & GDB

The `openocd.cfg` should be modified based on your board (interface and target) and what you want it to do. If you plan on using your own printing functions (or the one we provide), make sure it properly initializes the serial connection.

The `gdbinit` file is purely optional but can make the flashing and debugging process more enjoyable. 


## Generating your board's library

Library generation is done within the `libraryGeneration/` folder. The `libgen/main.py` script generates abstraction functions based on the provided `.json` and jinja2 templates file. Extra files may also be given to the script so that it automatically adds them to the generated library.

### Description files

In order for the library generator to work, you must provide the proper register description through json files. The template json files provide an example of what these description files should look like.

There are two types of descriptions: simple and exhaustive. These refer to the complexity of the compenonts of the board. Some compononts are simple in the sense that every components posses the same registers (GPIOs for example), while some are complexe in the sense that the available registers vary based on the component (TIM1 vs TIM2 for example).

The provided json files were mostly written based on the [ST RM0090 Reference manual](https://www.st.com/content/ccc/resource/technical/document/reference_manual/3d/6d/5a/66/b4/99/40/d4/DM00031020.pdf/files/DM00031020.pdf/jcr:content/translations/en.DM00031020.pdf). Similar manuals also exist for other boards on the ST website.


For each json description file, you must also provide a jinja2 template file in the `template/`. New templates should ideally include the following lines :
```rs
extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various::*;
{% include "address.rs" %}
```

The `address.rs` jinja template provides the following generated code :
- bases addresses ;
- registers offsets ;
- component_register_write() functions ;
- component_register_read() functions ;
- component_register_set() functions ;
- component_register_seti() functions ;

These elements should be the same for most components. You may also add component-specific functions within this jinja template file.


## Building projects

### src/main.rs 

Below is the minimum code required for a project to build :

```rs
#![no_std] //required option as std is not available on bare-metal application
#![no_main] //required option as std is not available on bare-metal application

extern crate geranium_rt; //required import to launch the lib.rs reset handler


#[no_mangle] // required macro for lib.rs to find where the main function is
fn main() {
    // init
    loop {
        // code 
    }
}
```

Generated modules can also be imported with :

```rs
use geranium_rt::stm32rustlib::gpio::*;
```

Should you wish to use interrupt handle functions, the following syntax must be followed (for our provided `lib.rs` anyway) :

```rs
#[no_mangle]
pub unsafe extern "C" fn handler() {
    ...
}

...

fn init() {
    ...

	nvic_handler_set(..., handler);

    ...
}

```



Various examples can also be found in the `libraryGeneration/unitaryTest/` folder.


### Flashing and debugging

Various rules are provided within the Makefile, but here are some explanations as to what commands to use :

#### openocd :
- in a first terminal, launch openocd to establish the connection to the board :
    > `openocd -f openocd.cfg`

#### logs :
- due to how we implemented our print function, you will need to track what gets writting in the log file in a second terminal :
    > `tail -f stm32f4.log`

    The path of this file can be specified in the openocd.cfg file

#### debug/flash
- the first step is to build the project. For that, simply type :
    > `cargo build`

    (You may also want to `cargo clean` before hand just in case, it often solves problems)
- next, flash the board with your GDB :
    > `gdb -q target/targetname/debug/projectname`
    
    For example : 

    > `gdb -q target/thumbv7em-none-eabi/debug/app`

    Once in the GDB, you will need to at least use the following commands :

    > `target remote localhost:3333`

    > `load`
    
    After that, you will be debugging as you would debug any other project. You should easily be able to find various tutorials online on how to use the debugging tool.


Note : if you attempt to flash with `cargo build --release`, you may be unable to debug due to the compiler optimizing the executable.

## Use the shceduler
See the [scheduling documentation](./taskSequencing/README.md)
