#![feature(restricted_std)]
#![allow(unused_imports)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![feature(is_some_and)]

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

mod threads_tests;

mod my_threads_tests;


#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustTickerTask() {

    thread::spawn(threads_tests);

}

fn threads_tests() {

    my_threads_tests::run_all_tests();



    println!("All tests done successfully !!");

}
