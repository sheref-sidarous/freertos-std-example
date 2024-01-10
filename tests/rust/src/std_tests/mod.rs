#![allow(dead_code)]


mod sync_barrier_tests;
mod sync_condvar_tests;
mod sync_mpsc_tests;
mod sync_mutex_tests;
mod sync_once_lock_tests;
mod sync_once_tests;
//mod sync_remutex_tests;
//mod sync_rwlock_tests;

pub mod sync {

    pub mod barrier {
        use crate::std_tests::sync_barrier_tests::*;

        pub fn all() {
            test_barrier();
        }
    }

    pub mod condvar {
        pub use crate::std_tests::sync_condvar_tests::*;

        pub fn all() {
            //smoke();
            notify_one();
            notify_all();
            //wait_while();
            //wait_timeout_wait();
            //wait_timeout_while_wait();
            //wait_timeout_while_instant_satisfy();
            //wait_timeout_while_wake();
            //wait_timeout_wake();
        }
    }
    pub mod mpsc {
        use crate::std_tests::sync_mpsc_tests::*;
        pub fn all() {
            smoke();
            drop_full();
            drop_full_shared();
            smoke_shared();
            smoke_threads();
            smoke_port_gone();
            smoke_shared_port_gone();
            smoke_shared_port_gone2();
            port_gone_concurrent();
            port_gone_concurrent_shared();
            smoke_chan_gone();
            smoke_chan_gone_shared();
            chan_gone_concurrent();
            //stress();
            //stress_shared();
            send_from_outside_runtime();
            recv_from_outside_runtime();
            no_runtime();
            oneshot_single_thread_close_port_first();
            oneshot_single_thread_close_chan_first();
            oneshot_single_thread_send_port_close();
            // oneshot_single_thread_recv_chan_close(); expected panic and capture
            oneshot_single_thread_send_then_recv();
            oneshot_single_thread_try_send_open();
            oneshot_single_thread_try_send_closed();
            oneshot_single_thread_try_recv_open();
        //}

        //fn remaining() {
            oneshot_single_thread_try_recv_closed();
            oneshot_single_thread_peek_data();
            oneshot_single_thread_peek_close();
            oneshot_single_thread_peek_open();
            oneshot_multi_task_recv_then_send();
            // oneshot_multi_task_recv_then_close(); expected panic and capture
            oneshot_multi_thread_close_stress();
            //oneshot_multi_thread_send_close_stress();
            //oneshot_multi_thread_recv_close_stress();
            //oneshot_multi_thread_send_recv_stress();
            //stream_send_recv_stress();
            oneshot_single_thread_recv_timeout();
            stress_recv_timeout_two_threads();
            recv_timeout_upgrade();
            stress_recv_timeout_shared();
            very_long_recv_timeout_wont_panic();
            recv_a_lot();
            shared_recv_timeout();
            //shared_chan_stress();
            test_nested_recv_iter();
            test_recv_iter_break();
            test_recv_try_iter();
            test_recv_into_iter_owned();
            test_recv_into_iter_borrowed();
            try_recv_states();
            destroy_upgraded_shared_port_when_sender_still_active();
            issue_32114();
            issue_39364();
        }
    }
    pub mod mutex {
        use crate::std_tests::sync_mutex_tests::*;

        pub fn all() {
            smoke();
            lots_and_lots();
            try_lock();
            test_case_into_inner();
            test_case_into_inner_drop();
            //test_case_into_inner_poison(); // needs panic-unwind
            test_case_get_mut();
            //test_case_get_mut_poison();
            test_case_mutex_arc_condvar();
            //test_case_arc_condvar_poison();
            //test_case_mutex_arc_poison();
            test_case_mutex_arc_nested();
            //test_case_mutex_arc_access_in_unwind();
            test_case_mutex_unsized();
        }
    }

    pub mod once_lock {
        use crate::std_tests::sync_once_lock_tests::*;

        pub fn all() {
                sync_once_cell();
                sync_once_cell_get_mut();
                //sync_once_cell_get_unchecked();
                sync_once_cell_drop();
                sync_once_cell_drop_empty();
                clone();
                //get_or_try_init();
                from_impl();
                partialeq_impl();
                into_inner();
                is_sync_send();
                eval_once_macro();
                sync_once_cell_does_not_leak_partially_constructed_boxes();
                dropck();
        }
    }
    pub mod once {
        use crate::std_tests::sync_once_tests::*;

        pub fn all() {
            smoke_once();
            stampede_once();
            //poison_bad();
            //wait_for_force_to_finish();
        }
    }

/*
    pub mod remutex {
        use crate::std_tests::sync_remutex_tests::*;


    }

    pub mod rwlock {
        use crate::std_tests::sync_rwlock_tests::*;

    }
*/
}