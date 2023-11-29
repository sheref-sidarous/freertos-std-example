#![allow(dead_code)]

mod sync_mutex_tests;
mod sync_barrier_tests;

pub mod sync {
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

    pub mod barrier {

        use crate::std_tests::sync_barrier_tests::*;

        pub fn all() {
            test_barrier();
        }
    }


}