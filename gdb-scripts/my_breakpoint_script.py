import gdb


# Define a function to run when the breakpoint is hit
def alloc_breakpoint_handler():
    size = gdb.parse_and_eval ('layout.size()')
    align = gdb.parse_and_eval ('layout.align()')
    trace.write(f"{size}, {align}\n")
    trace.flush()
    # You can add your custom code here

def free_breakpoint_handler():
    size = gdb.parse_and_eval ('_layout.size()') * -1
    align = gdb.parse_and_eval ('_layout.align()') * -1
    trace.write(f"{size}, {align}\n")
    trace.flush()

trace = open("alloc_trace.csv", "w")

# Set up the breakpoint and associate the handler function
#breakpoint = gdb.Breakpoint("freertos_std::sys::freertos::alloc::<impl core::alloc::global::GlobalAlloc for freertos_std::alloc::System>::alloc")
alloc_breakpoint = gdb.Breakpoint("src/sys/freertos/alloc.rs:44")
# Add custom actions to the breakpoint using the 'commands' command
alloc_breakpoint.commands = \
    "python alloc_breakpoint_handler()\n"\
    "continue"

free_breakpoint = gdb.Breakpoint("src/sys/freertos/alloc.rs:57")
# Add custom actions to the breakpoint using the 'commands' command
free_breakpoint.commands = \
    "python free_breakpoint_handler()\n"\
    "continue"


# Optional: Disable the breakpoint after it's hit once
# breakpoint.silent = True
# breakpoint.continue_after_one = True
