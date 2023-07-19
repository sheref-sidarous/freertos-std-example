#![feature(restricted_std)]
#![allow(unused_imports)]
#![feature(core_panic)]
#![feature(fmt_internals)]

// use freertos_rust::*;
use core::ffi::c_char;
use core::ffi::c_int;

use std::thread;
use std::time::Duration;
use std::time::Instant;
use core::panicking::panic_fmt;
use core::fmt::Arguments;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Sender},
    Arc, Barrier,
};

extern "C" {
    fn printf(fmt: *const u8, ...) -> c_int;
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustTickerTask() {

    thread::spawn( move || {
        loop {
            println!("Rust Tick! <3");
            thread::sleep(Duration::from_millis(1000));
        }
    });
}
