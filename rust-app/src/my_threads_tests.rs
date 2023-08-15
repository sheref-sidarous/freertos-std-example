

use std::sync::Mutex;
/*
**************** Testing threads
*/
use std::thread;
use std::time;
use std::cell::RefCell;
use std::sync::{self, Arc, RwLock};

// Test join : that a parent thread will block until the child thread is terminanted with a value, and the value is correct
fn test_join() {
    let x = 2;
    let thread_handle = thread::spawn(move || {
        x * 2
    });
    let result = thread_handle.join().unwrap();
    assert_eq!(result, 4);
}

// Test thread sleep
fn test_sleep() {
    let before = time::Instant::now();
    let duration = time::Duration::from_millis(100);
    thread::sleep(duration);
    let after = time::Instant::now();
    assert!(after >= before + duration)
}

// Test thread yield


// Test thread local storage
fn test_thread_local_storage() {
    // Taken from std LocalKey documentation
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // each thread starts out with the initial value of 1
    let t = thread::spawn(move|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });

    // wait for the thread to complete and bail out on panic
    t.join().unwrap();

    // we retain our original value of 2 despite the child thread
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}


/*
Allocation testing
*/




/*
Test Sync
 */

// Mutex
fn test_mutex() {

    let counter = Arc::new(Mutex::new(0)); // Wrap the counter in an Arc and Mutex

    let mutator1_counter = counter.clone(); // Clone the Arc for mutator thread 1
    let mutator2_counter = counter.clone(); // Clone the Arc for mutator thread 2

    let mutator1 = thread::spawn(move || {
        for _ in 0..5 {
            let mut count = mutator1_counter.lock().unwrap(); // Lock the Mutex
            *count += 1;
            // The Mutex is automatically unlocked when 'count' goes out of scope
        }
    });

    let mutator2 = thread::spawn(move || {
        for _ in 0..5 {
            let mut count = mutator2_counter.lock().unwrap(); // Lock the Mutex
            *count += 1;
            // The Mutex is automatically unlocked when 'count' goes out of scope
        }
    });

    mutator1.join().unwrap();
    mutator2.join().unwrap();



    assert_eq!(*counter.lock().unwrap(), 10);



}

// TODO: a test Once where one of the functions panics
fn test_Once () {

    static mut UNPROTECED_SHARED_VAR : i32 = 0;
    static INCREMENT_ONCE : sync::Once = sync::Once::new();

    let long_winded_increment = || {
        thread::sleep(time::Duration::from_millis(100));
        unsafe {
            UNPROTECED_SHARED_VAR += 1;
        }
        thread::sleep(time::Duration::from_millis(100))
    };
    let long_winded_increment_clone = long_winded_increment.clone();

    let thread_handle = thread::spawn( move || {
        INCREMENT_ONCE.call_once(long_winded_increment_clone);
    });

    INCREMENT_ONCE.call_once(long_winded_increment);

    thread_handle.join().unwrap();

    unsafe {
        assert_eq!(UNPROTECED_SHARED_VAR, 1);
    }

}

// Read / Write Locks
fn test_RW_lock() {

    let data = Arc::new(RwLock::new(0)); // Wrap the data in an Arc and RwLock

    let reader1_data = data.clone(); // Clone the Arc for reader thread 1
    let reader2_data = data.clone(); // Clone the Arc for reader thread 2
    let writer_data = data.clone();  // Clone the Arc for writer thread

    let reader1 = thread::spawn(move || {
        let reader = reader1_data.read().unwrap(); // Read lock
        println!("Reader 1: Data = {}", *reader);
        // The read lock is automatically released when 'reader' goes out of scope
    });

    let reader2 = thread::spawn(move || {
        let reader = reader2_data.read().unwrap(); // Read lock
        println!("Reader 2: Data = {}", *reader);
        // The read lock is automatically released when 'reader' goes out of scope
    });

    let writer = thread::spawn(move || {
        let mut data = writer_data.write().unwrap(); // Write lock
        *data += 42;
        println!("Writer: Data modified to {}", *data);
        // The write lock is automatically released when 'data' goes out of scope
    });

    reader1.join().unwrap();
    reader2.join().unwrap();
    writer.join().unwrap();

}

 // Conditional variables

 // send/recv

 pub fn run_all_tests() {

    //test_join();

    //test_sleep();
    //test_thread_local_storage();

    //test_mutex();
    test_RW_lock();

    //test_Once();

}