# Rust -> STM32

Providing a way to compile any Rust program for STM32

## Installation

Install rust by following the instructions at [rustc] (https://rustup.rs)

In a bare metal environement, we can not load the standard library. To prevent rust from loading the library use **no_std**. 

Or you can install cargo no-std-check which ensure thet your library does not link to **libstd** :
 
> **cargo no-std-check**

Then run this command on a crate to build it's lib target without access to **std** :

> **cargo no-std-check --manifest-path nostd/cargo.toml**

## Installing tools

Make sure you have a compiler version equal or newer than 1.31.

>$ rustc -V 