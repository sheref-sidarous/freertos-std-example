

use std::sync::Mutex;
/*
**************** Testing threads
*/
use std::thread;
use std::time;
use std::cell::RefCell;
use std::sync::{self, Arc, RwLock, Condvar, mpsc};


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
fn test_once () {

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
fn test_rw_lock_1() {

    let data = Arc::new(RwLock::new(10)); // Wrap the data in an Arc and RwLock

    let reader_data = data.clone(); // Clone the Arc for reader thread
    let writer_data = data.clone();  // Clone the Arc for writer thread

    // Hold a reader lock in the current thread
    let read_data = data.read().unwrap();

    // this reader thread should proceed anyway
    let reader1 = thread::spawn(move || {
        let reader = reader_data.read().unwrap(); // Read lock
        assert_eq!(*reader, 10);
    });

    reader1.join().unwrap();

    // then, try to hold a writer thread
    let writer = thread::spawn(move || {
        let mut data = writer_data.write().unwrap(); // Write lock
        *data += 42;
        assert_eq!(*data, 52);
    });

    // sleep for a bit to allow writer some time
    thread::sleep(time::Duration::from_millis(100));

    // writer thread SHOULD NOT be finished
    assert!(!writer.is_finished());

    // and reader should still read 10
    assert_eq!(*read_data, 10);

    // release the read lock
    drop(read_data);

    // Now, writer should be able to finish
    writer.join().unwrap();

    // if I acquired another Read lock is should read 52
    let read_data_again = data.read().unwrap();
    assert_eq!(*read_data_again, 52);

}

fn test_rw_lock_2() {

    let data = Arc::new(RwLock::new(10)); // Wrap the data in an Arc and RwLock

    let reader_data = data.clone(); // Clone the Arc for reader thread

    // Hold a writer lock in the current thread
    let mut writer = data.write().unwrap();

    // this reader thread should block until writer is over
    let reader_thread = thread::spawn(move || {
        let reader = reader_data.read().unwrap(); // Read lock
        assert_eq!(*reader, 52);
    });

    // sleep for a bit to allow the thread some time
    thread::sleep(time::Duration::from_millis(100));

    // writer thread SHOULD NOT be finished
    assert!(!reader_thread.is_finished());

    // write and release the write lock
    *writer += 42;
    drop(writer);

    // Now, reader should be able to finish
    reader_thread.join().unwrap();


}


// Conditional variables
fn test_condvar() {
    // Create shared data between threads
    let data = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());

    // Clone the data and condvar for use in multiple threads
    let data_clone = Arc::clone(&data);
    let condvar_clone = Arc::clone(&condvar);

    // Create a thread that waits for the condition to be true
    let thread1 = thread::spawn(move || {
        let mut locked_data = data_clone.lock().unwrap();
        while !*locked_data {
            locked_data = condvar_clone.wait(locked_data).unwrap();
        }
        assert!(*locked_data);
    });

    // sleep for a bit to allow the thread some time
    thread::sleep(time::Duration::from_millis(100));

    // thread1 SHOULD NOT be finished
    assert!(!thread1.is_finished());

    // Create another thread that modifies the condition and signals the first thread
    let thread2 = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(100)); // Simulate some work
        let mut locked_data = data.lock().unwrap();
        *locked_data = true;
        condvar.notify_one();
    });

    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
}

// send/recv
fn test_mpsc() {
    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread that sends messages
    let sender_thread = thread::spawn(move || {
        for i in 0..5 {
            sender.send(i).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Receive messages in the main thread
    let mut count_verify = 0;
    for received in receiver {
        assert_eq!(received, count_verify);
        count_verify += 1;
    }

    // Wait for the sender thread to finish
    sender_thread.join().unwrap();
}


pub fn run_all_tests() {

    test_join();

    test_sleep();
    test_thread_local_storage();

    test_mutex();
    test_once();
    test_rw_lock_1();
    test_rw_lock_2();

    test_condvar();
    test_mpsc();

    println!("All Sync tests ran successfully !!");
}