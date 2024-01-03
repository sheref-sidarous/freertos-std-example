set print pretty on
set pagination off
python import freertos_gdb
target remote localhost:1234
file output/RTOSDemo.out
set substitute-path ../../../../../../libgcc/ /home/shiro/temp/gcc-arm-none-eabi-source/libgcc
b prvGetRegistersFromStack
b panic_fmt
b panic_misaligned_pointer_dereference
b panic_bounds_check
b abort_internal
b panic_nounwind_fmt
b freertos_std::panicking::rust_panic_with_hook
b __rust_start_panic
b _Unwind_RaiseException
b get_eit_entry
b rust_panic
#source ../gdb-scripts/my_breakpoint_script.py