#![feature(restricted_std)]
#![allow(unused_imports)]

#![allow(dead_code)]
#![feature(once_cell_try)]

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
    OnceLock,
};

use cortex_m_semihosting::debug::{self, EXIT_SUCCESS, EXIT_FAILURE};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustEntryFunction() {

    thread::spawn(main_thread);

}

fn main_thread() {

    for i in 0..10 {

        println!("Rust Tick {}", i);
        thread::sleep(Duration::from_millis(1000))

    }

    println!("Bye Bye");


    debug::exit(EXIT_SUCCESS);


}
