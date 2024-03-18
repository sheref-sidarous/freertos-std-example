use std::thread::Builder;
use std::any::Any;
use std::mem;
use std::panic::panic_any;
use std::result;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Sender},
    Arc, Barrier,
};
use std::thread::{self, Scope, ThreadId};
use std::time::Duration;
use std::time::Instant;

// !!! These tests are dangerous. If something is buggy, they will hang, !!!
// !!! instead of exiting cleanly. This might wedge the buildbots.       !!!

pub fn test_unnamed_thread() {
    thread::spawn(move || {
        assert!(thread::current().name().is_none());
    })
    .join()
    .ok()
    .expect("thread panicked");
}

pub fn test_named_thread() {
    Builder::new()
        .name("ada lovelace".to_string())
        .spawn(move || {
            assert!(thread::current().name().unwrap() == "ada lovelace".to_string());
        })
        .unwrap()
        .join()
        .unwrap();
}

#[cfg(any(
    // Note: musl didn't add pthread_getname_np until 1.2.3
    all(target_os = "linux", target_env = "gnu"),
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "watchos"
))]
pub fn test_named_thread_truncation() {
    use std::ffi::CStr;

    let long_name = std::iter::once("test_named_thread_truncation")
        .chain(std::iter::repeat(" yada").take(100))
        .collect::<String>();

    let result = Builder::new().name(long_name.clone()).spawn(move || {
        // Rust remembers the full thread name itself.
        assert_eq!(thread::current().name(), Some(long_name.as_str()));

        // But the system is limited -- make sure we successfully set a truncation.
        let mut buf = vec![0u8; long_name.len() + 1];
        unsafe {
            libc::pthread_getname_np(libc::pthread_self(), buf.as_mut_ptr().cast(), buf.len());
        }
        let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
        assert!(cstr.to_bytes().len() > 0);
        assert!(long_name.as_bytes().starts_with(cstr.to_bytes()));
    });
    result.unwrap().join().unwrap();
}

#[should_panic]
pub fn test_invalid_named_thread() {
    let _ = Builder::new().name("ada l\0velace".to_string()).spawn(|| {});
}

pub fn test_run_basic() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(()).unwrap();
    });
    rx.recv().unwrap();
}

pub fn test_is_finished() {
    let b = Arc::new(Barrier::new(2));
    let t = thread::spawn({
        let b = b.clone();
        move || {
            b.wait();
            1234
        }
    });

    // Thread is definitely running here, since it's still waiting for the barrier.
    assert_eq!(t.is_finished(), false);

    // Unblock the barrier.
    b.wait();

    // Now check that t.is_finished() becomes true within a reasonable time.
    let start = Instant::now();
    while !t.is_finished() {
        assert!(start.elapsed() < Duration::from_secs(2));
        thread::sleep(Duration::from_millis(15));
    }

    // Joining the thread should not block for a significant time now.
    let join_time = Instant::now();
    assert_eq!(t.join().unwrap(), 1234);
    assert!(join_time.elapsed() < Duration::from_secs(2));
}

pub fn test_join_panic() {
    match thread::spawn(move || panic!()).join() {
        result::Result::Err(_) => (),
        result::Result::Ok(()) => panic!(),
    }
}

pub fn test_spawn_sched() {
    let (tx, rx) = channel();

    fn f(i: i32, tx: Sender<()>) {
        let tx = tx.clone();
        thread::spawn(move || {
            if i == 0 {
                tx.send(()).unwrap();
            } else {
                f(i - 1, tx);
            }
        });
    }
    f(10, tx);
    rx.recv().unwrap();
}

pub fn test_spawn_sched_childs_on_default_sched() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        thread::spawn(move || {
            tx.send(()).unwrap();
        });
    });

    rx.recv().unwrap();
}

fn avoid_copying_the_body<F>(spawnfn: F)
where
    F: FnOnce(Box<dyn Fn() + Send>),
{
    let (tx, rx) = channel();

    let x: Box<_> = Box::new(1);
    let x_in_parent = (&*x) as *const i32 as usize;

    spawnfn(Box::new(move || {
        let x_in_child = (&*x) as *const i32 as usize;
        tx.send(x_in_child).unwrap();
    }));

    let x_in_child = rx.recv().unwrap();
    assert_eq!(x_in_parent, x_in_child);
}

pub fn test_avoid_copying_the_body_spawn() {
    avoid_copying_the_body(|v| {
        thread::spawn(move || v());
    });
}

pub fn test_avoid_copying_the_body_thread_spawn() {
    avoid_copying_the_body(|f| {
        thread::spawn(move || {
            f();
        });
    })
}

pub fn test_avoid_copying_the_body_join() {
    avoid_copying_the_body(|f| {
        let _ = thread::spawn(move || f()).join();
    })
}

pub fn test_child_doesnt_ref_parent() {
    // Reduced to 4 for FreeRTOS
    const GENERATIONS: u32 = 4;
    fn child_no(x: u32) -> Box<dyn Fn() + Send> {
        return Box::new(move || {
            if x < GENERATIONS {
                thread::spawn(move || child_no(x + 1)());
            }
        });
    }
    thread::spawn(|| child_no(0)());
}

pub fn test_simple_newsched_spawn() {
    thread::spawn(move || {});
}

pub fn test_try_panic_message_string_literal() {
    match thread::spawn(move || {
        panic!("static string");
    })
    .join()
    {
        Err(e) => {
            type T = &'static str;
            assert!(e.is::<T>());
            assert_eq!(*e.downcast::<T>().unwrap(), "static string");
        }
        Ok(()) => panic!(),
    }
}

pub fn test_try_panic_any_message_owned_str() {
    match thread::spawn(move || {
        panic_any("owned string".to_string());
    })
    .join()
    {
        Err(e) => {
            type T = String;
            assert!(e.is::<T>());
            assert_eq!(*e.downcast::<T>().unwrap(), "owned string".to_string());
        }
        Ok(()) => panic!(),
    }
}

pub fn test_try_panic_any_message_any() {
    match thread::spawn(move || {
        panic_any(Box::new(413u16) as Box<dyn Any + Send>);
    })
    .join()
    {
        Err(e) => {
            type T = Box<dyn Any + Send>;
            assert!(e.is::<T>());
            let any = e.downcast::<T>().unwrap();
            assert!(any.is::<u16>());
            assert_eq!(*any.downcast::<u16>().unwrap(), 413);
        }
        Ok(()) => panic!(),
    }
}

pub fn test_try_panic_any_message_unit_struct() {
    struct Juju;

    match thread::spawn(move || panic_any(Juju)).join() {
        Err(ref e) if e.is::<Juju>() => {}
        Err(_) | Ok(()) => panic!(),
    }
}

pub fn test_park_unpark_before() {
    for _ in 0..10 {
        thread::current().unpark();
        thread::park();
    }
}

pub fn test_park_unpark_called_other_thread() {
    for _ in 0..10 {
        let th = thread::current();

        let _guard = thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(50));
            th.unpark();
        });

        thread::park();
    }
}

pub fn test_park_timeout_unpark_before() {
    for _ in 0..10 {
        thread::current().unpark();
        thread::park_timeout(Duration::from_millis(u32::MAX as u64));
    }
}

pub fn test_park_timeout_unpark_not_called() {
    for _ in 0..10 {
        thread::park_timeout(Duration::from_millis(10));
    }
}

pub fn test_park_timeout_unpark_called_other_thread() {
    for _ in 0..10 {
        let th = thread::current();

        let _guard = thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(50));
            th.unpark();
        });

        thread::park_timeout(Duration::from_millis(u32::MAX as u64));
    }
}

pub fn sleep_ms_smoke() {
    thread::sleep(Duration::from_millis(2));
}

pub fn test_size_of_option_thread_id() {
    assert_eq!(mem::size_of::<Option<ThreadId>>(), mem::size_of::<ThreadId>());
}

pub fn test_thread_id_equal() {
    assert!(thread::current().id() == thread::current().id());
}

pub fn test_thread_id_not_equal() {
    let spawned_id = thread::spawn(|| thread::current().id()).join().unwrap();
    assert!(thread::current().id() != spawned_id);
}

pub fn test_scoped_threads_drop_result_before_join() {
    let actually_finished = &AtomicBool::new(false);
    struct X<'scope, 'env>(&'scope Scope<'scope, 'env>, &'env AtomicBool);
    impl Drop for X<'_, '_> {
        fn drop(&mut self) {
            thread::sleep(Duration::from_millis(20));
            let actually_finished = self.1;
            self.0.spawn(move || {
                thread::sleep(Duration::from_millis(20));
                actually_finished.store(true, Ordering::Relaxed);
            });
        }
    }
    thread::scope(|s| {
        s.spawn(move || {
            thread::sleep(Duration::from_millis(20));
            X(s, actually_finished)
        });
    });
    assert!(actually_finished.load(Ordering::Relaxed));
}

pub fn test_scoped_threads_nll() {
    // this is mostly a *compilation test* for this exact function:
    fn foo(x: &u8) {
        thread::scope(|s| {
            s.spawn(|| match x {
                _ => (),
            });
        });
    }
    // let's also run it for good measure
    let x = 42_u8;
    foo(&x);
}

// Regression test for https://github.com/rust-lang/rust/issues/98498.
#[cfg(miri)] // relies on Miri's data race detector
pub fn scope_join_race() {
    for _ in 0..100 {
        let a_bool = AtomicBool::new(false);

        thread::scope(|s| {
            for _ in 0..5 {
                s.spawn(|| a_bool.load(Ordering::Relaxed));
            }

            for _ in 0..5 {
                s.spawn(|| a_bool.load(Ordering::Relaxed));
            }
        });
    }
}
