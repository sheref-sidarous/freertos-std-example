#![feature(restricted_std)]
#![allow(unused_imports)]
#![feature(fmt_internals)]
#![feature(is_some_and)]

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


mod sync_tests;


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustTickerTask() {

    thread::spawn(main_thread);

    thread::spawn(move || {
        loop {
            println!("Rust Tick! <3");
            thread::sleep(Duration::from_millis(1000));
        }
    });

}

fn main_thread() {

    sync_tests::run_all_tests();
    panic!("test panic");

}