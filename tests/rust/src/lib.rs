#![feature(restricted_std)]
#![allow(unused_imports)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![allow(dead_code)]
#![feature(once_cell_try)]

use std::thread;


use cortex_m_semihosting::debug::{self, EXIT_SUCCESS, EXIT_FAILURE};

mod sync_tests;
mod std_tests;

mod test_function;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn vRustEntryFunction() {

    thread::spawn(|| {
        test_function::FUNCTION();
        println!("Test function ran successfully");
        debug::exit(EXIT_SUCCESS);
    });

}
