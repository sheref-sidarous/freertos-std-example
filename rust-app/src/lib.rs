#![feature(restricted_std)]
#![allow(unused_imports)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![allow(dead_code)]
#![feature(once_cell_try)]

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


mod sync_tests;
mod std_tests;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustTickerTask() {

    thread::spawn(main_thread);

}

fn main_thread() {

    thread::spawn( || {
        sync_tests::run_all_tests();
    }).join();

    //std_tests::sync::barrier::all(); // PASS
    std_tests::sync::condvar::all(); // PASS
    //std_tests::sync::mpsc::all(); // fails malloc
    //std_tests::sync::mutex::all(); // PASS
    //std_tests::sync::once_lock::all();  // fails malloc
    //std_tests::sync::once::all(); // PASS after panic unwind tests are disabled
    //std_tests::sync::remutex::all(); // not ported yet
    //std_tests::sync::rwlock::all(); // not ported yet

    println!("All tests are done!");


}
