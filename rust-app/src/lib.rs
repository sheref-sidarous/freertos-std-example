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
use threads_tests::*;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustTickerTask() {

    thread::spawn(threads_tests);

}

fn threads_tests() {

    test_unnamed_thread();
    //test_named_thread();
    //test_invalid_named_thread();
    test_run_basic();
    test_is_finished();
    println!("All tests done successfully !!");
    /*
    test_join_panic();
    test_spawn_sched();
    test_spawn_sched_childs_on_default_sched();
    test_avoid_copying_the_body_spawn();
    test_avoid_copying_the_body_thread_spawn();
    test_avoid_copying_the_body_join();
    test_child_doesnt_ref_parent();
    test_simple_newsched_spawn();
    test_try_panic_message_string_literal();
    test_try_panic_any_message_owned_str();
    test_try_panic_any_message_any();
    test_try_panic_any_message_unit_struct();
    test_park_unpark_before();
    test_park_unpark_called_other_thread();
    test_park_timeout_unpark_before();
    test_park_timeout_unpark_not_called();
    test_park_timeout_unpark_called_other_thread();
    test_size_of_option_thread_id();
    test_thread_id_equal();
    test_thread_id_not_equal();
    test_scoped_threads_drop_result_before_join();
    test_scoped_threads_nll();
    */

}
