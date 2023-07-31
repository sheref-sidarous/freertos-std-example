# A usage example for freertos-std

This is an example of how to build and use [freertos-std](https://github.com/sheref-sidarous/freertos-std), A Rust Standard Library implementation for FreeRTOS

## Directory structure:
* FreeRTOS-Kernel: FreeRTOS source code (git submodule).
* FreeRTOS-Qemu-Demo: Here lives the demo is built and run, using Qemu.
* rust-app: The Rust example application.
* freertos-std: Rust's std implementation (git submodule).

## Dependencies
* Rust nightly
* ARM GCC compiler
* Qemu

## Building
```
$ cd FreeRTOS-Qemu-Demo
$ make
```

## Running
```
$ cd FreeRTOS-Qemu-Demo
$ qemu-system-arm -machine mps2-an385 -cpu cortex-m3 -kernel output/RTOSDemo.out -monitor none -nographic -serial stdio -s
```
