target remote localhost:1234
file output/RTOSDemo.out
b prvGetRegistersFromStack
b panic_fmt
b panic_misaligned_pointer_dereference
b panic_bounds_check
b abort_internal
b panic_nounwind_fmt
