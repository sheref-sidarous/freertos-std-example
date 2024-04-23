#![feature(restricted_std)]
#![allow(unused_imports)]

// use freertos_rust::*;
use core::ffi::c_char;
use core::ffi::c_int;

use std::thread;
use std::time::Duration;
use std::time::Instant;
use core::fmt::Arguments;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Sender},
    Arc, Barrier,
};




#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustEntryFunction() {

    thread::spawn(main_thread);

    thread::spawn(move || {
        loop {
            println!("Rust Tick! <3");
            thread::sleep(Duration::from_millis(1000));
        }
    });

}

fn main_thread() {

    panic!("test panic");

}