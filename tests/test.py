import pytest
import subprocess
import os

def prepend_mod_path(mod_path, fn_list) :
    return [ mod_path + "::" + fn_name for fn_name in fn_list ]

barrier_tests = [
    "std_tests::sync::barrier::test_barrier",
]

condvar_tests = [
    "std_tests::sync::condvar::smoke",
    "std_tests::sync::condvar::notify_one",
    "std_tests::sync::condvar::notify_all",
    "std_tests::sync::condvar::wait_while",
    "std_tests::sync::condvar::wait_timeout_wait",
    "std_tests::sync::condvar::wait_timeout_while_wait",
    "std_tests::sync::condvar::wait_timeout_while_instant_satisfy",
    "std_tests::sync::condvar::wait_timeout_while_wake",
    "std_tests::sync::condvar::wait_timeout_wake",
]

mpsc_tests = [
    "std_tests::sync::mpsc::smoke",
    "std_tests::sync::mpsc::drop_full",
    "std_tests::sync::mpsc::drop_full_shared",
    "std_tests::sync::mpsc::smoke_shared",
    "std_tests::sync::mpsc::smoke_threads",
    "std_tests::sync::mpsc::smoke_port_gone",
    "std_tests::sync::mpsc::smoke_shared_port_gone",
    "std_tests::sync::mpsc::smoke_shared_port_gone2",
    "std_tests::sync::mpsc::port_gone_concurrent",
    "std_tests::sync::mpsc::port_gone_concurrent_shared",
    "std_tests::sync::mpsc::smoke_chan_gone",
    "std_tests::sync::mpsc::smoke_chan_gone_shared",
    "std_tests::sync::mpsc::chan_gone_concurrent",
    "std_tests::sync::mpsc::stress",
    "std_tests::sync::mpsc::stress_shared",
    "std_tests::sync::mpsc::send_from_outside_runtime",
    "std_tests::sync::mpsc::recv_from_outside_runtime",
    "std_tests::sync::mpsc::no_runtime",
    "std_tests::sync::mpsc::oneshot_single_thread_close_port_first",
    "std_tests::sync::mpsc::oneshot_single_thread_close_chan_first",
    "std_tests::sync::mpsc::oneshot_single_thread_send_port_close",
    # "std_tests::sync::mpsc::oneshot_single_thread_recv_chan_close", needs panic unwind
    "std_tests::sync::mpsc::oneshot_single_thread_send_then_recv",
    "std_tests::sync::mpsc::oneshot_single_thread_try_send_open",
    "std_tests::sync::mpsc::oneshot_single_thread_try_send_closed",
    "std_tests::sync::mpsc::oneshot_single_thread_try_recv_open",
    "std_tests::sync::mpsc::oneshot_single_thread_try_recv_closed",
    "std_tests::sync::mpsc::oneshot_single_thread_peek_data",
    "std_tests::sync::mpsc::oneshot_single_thread_peek_close",
    "std_tests::sync::mpsc::oneshot_single_thread_peek_open",
    "std_tests::sync::mpsc::oneshot_multi_task_recv_then_send",
    # "std_tests::sync::mpsc::oneshot_multi_task_recv_then_close", needs panic unwind
    "std_tests::sync::mpsc::oneshot_multi_thread_close_stress",
    #"std_tests::sync::mpsc::oneshot_multi_thread_send_close_stress", needs panic unwind
    "std_tests::sync::mpsc::oneshot_multi_thread_recv_close_stress",
    "std_tests::sync::mpsc::oneshot_multi_thread_send_recv_stress",
    "std_tests::sync::mpsc::stream_send_recv_stress",
    "std_tests::sync::mpsc::oneshot_single_thread_recv_timeout",
    "std_tests::sync::mpsc::stress_recv_timeout_two_threads",
    "std_tests::sync::mpsc::recv_timeout_upgrade",
    "std_tests::sync::mpsc::stress_recv_timeout_shared",
    "std_tests::sync::mpsc::very_long_recv_timeout_wont_panic",
    "std_tests::sync::mpsc::recv_a_lot",
    "std_tests::sync::mpsc::shared_recv_timeout",
    "std_tests::sync::mpsc::shared_chan_stress",
    "std_tests::sync::mpsc::test_nested_recv_iter",
    "std_tests::sync::mpsc::test_recv_iter_break",
    "std_tests::sync::mpsc::test_recv_try_iter",
    "std_tests::sync::mpsc::test_recv_into_iter_owned",
    "std_tests::sync::mpsc::test_recv_into_iter_borrowed",
    "std_tests::sync::mpsc::try_recv_states",
    "std_tests::sync::mpsc::destroy_upgraded_shared_port_when_sender_still_active",
    "std_tests::sync::mpsc::issue_32114",
    "std_tests::sync::mpsc::issue_39364",
]

mutex_tests = [ "std_tests::sync::mutex::" + fn_name for fn_name in [
    "smoke",
    "lots_and_lots",
    "try_lock",
    "test_case_into_inner",
    "test_case_into_inner_drop",
    # "test_case_into_inner_poison", needs panic unwind
    "test_case_get_mut",
    #"test_case_get_mut_poison", needs panic unwind
    "test_case_mutex_arc_condvar",
    #"test_case_arc_condvar_poison", needs panic unwind
    #"test_case_mutex_arc_poison", needs panic unwind
    "test_case_mutex_arc_nested",
    #"test_case_mutex_arc_access_in_unwind", needs panic unwind
    "test_case_mutex_unsized",
]]

once_lock_tests = [ "std_tests::sync::once_lock::" + fn_name for fn_name in [
    "sync_once_cell",
    "sync_once_cell_get_mut",
    # "sync_once_cell_get_unchecked", relies on internal API
    "sync_once_cell_drop",
    "sync_once_cell_drop_empty",
    "clone",
    #"get_or_try_init", relies on internal API and panic unwind
    "from_impl",
    "partialeq_impl",
    "into_inner",
    "is_sync_send",
    "eval_once_macro",
    "sync_once_cell_does_not_leak_partially_constructed_boxes",
    "dropck",
]]

once_tests = [ "std_tests::sync::once::" + fn_name for fn_name in [
    "smoke_once",
    "stampede_once",
    "poison_bad",
    "wait_for_force_to_finish",
]]

remutex_tests = prepend_mod_path("std_tests::sync::remutex", [
    "smoke",
    "is_mutex",
    "trylock_works",
])

rwlock_tests = prepend_mod_path("std_tests::sync::rwlock", [
    "smoke",
    "frob",
    "test_rw_arc_poison_wr",
    "test_rw_arc_poison_ww",
    "test_rw_arc_no_poison_rr",
    "test_rw_arc_no_poison_rw",
    "test_rw_arc",
    "test_rw_arc_access_in_unwind",
    "test_rwlock_unsized",
    "test_rwlock_try_write",
    "test_into_inner",
    "test_into_inner_drop",
    "test_into_inner_poison",
    "test_get_mut",
    "test_get_mut_poison",
    "test_read_guard_covariance",
])

extra_sync_tests = prepend_mod_path("sync_tests", [
    "test_join",
    "test_sleep",
    "test_thread_local_storage",
    "test_mutex",
    "test_once ",
    "test_rw_lock_1",
    "test_rw_lock_2",
    "test_condvar",
    "test_mpsc",
    "run_all_tests",
])

all_tests = barrier_tests + condvar_tests + mpsc_tests + mutex_tests + once_lock_tests + extra_sync_tests #+ remutex_tests + rwlock_tests

@pytest.mark.parametrize("test_function", all_tests)
def test_function(test_function):
    test_function_path = "rust/src/test_function.rs"
    #if os.path.exists(test_function_path):  # Check if the file exists
    #      # Delete the file
    with open(test_function_path, "w") as fh:
        fh.write(f"pub static FUNCTION : fn() = crate::{test_function};\n")

    build_result = subprocess.run("RUST_APP_ROOT=../tests/rust make all", cwd="../FreeRTOS-Qemu-Demo", shell=True,
                   capture_output=True, text=True)

    if build_result.returncode != 0:
        print("Build Failed :\n")
        print(build_result.stderr)
        raise Exception("Build failed")

    run_result = subprocess.run(["qemu-system-arm",
                    "-machine", "mps2-an385",
                    "-cpu", "cortex-m3",
                    "-kernel", "output/RTOSDemo.out",
                    "-monitor", "none ",
                    "-nographic",
                    "-serial", "stdio",
                    "-s",
                    "--semihosting-config", "enable=on,target=native"], cwd="../FreeRTOS-Qemu-Demo",
                    capture_output=True, text=True)


    #os.remove(test_function_path)

    print("============================== QEMU stderr ==============================")
    print(run_result.stdout)
    print("============================== QEMU stdout ==============================")
    print(run_result.stdout)

    assert(run_result.returncode == 0), "Qemu exited with non-zero status"
