
use std::thread;
use std::time::Duration;

use cortex_m_semihosting::debug::{self, EXIT_SUCCESS};


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
