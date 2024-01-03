import gdb

#global freed_allocs
freed_allocs = []

#global active_allocs
active_allocs = {}

orphan_frees = []

class AllocationDescriptor:
    def __init__(self):
        self.alloc_backtrace = None
        self.dealloc_backtrace = None
        self.addr = None
        self.fault = False

    def to_json(self):
        return {
            "alloc_backtrace": self.alloc_backtrace,
            "dealloc_backtrace": self.dealloc_backtrace,
            "addr": self.addr,
            "fault": self.fault,
        }

class AllocDoneBreakpoint (gdb.FinishBreakpoint):

    def __init__(self, alloc_desc):
        gdb.FinishBreakpoint.__init__ (self)
        self.alloc_desc = alloc_desc

    def stop (self):
        addr = int(self.return_value) #cast(gdb.lookup_type("int")).string()
        self.alloc_desc.addr = addr
        active_allocs[addr] = self.alloc_desc
        return False # False means continue

    def out_of_scope (self):
        print ("abnormal finish")
        gdb.execute("bt")
        self.fault = True

# Define a function to run when the breakpoint is hit
class AllocBreakpoint (gdb.Breakpoint):
    def stop(self):
        alloc_desc = AllocationDescriptor()
        alloc_desc.alloc_backtrace = gdb.execute("bt", to_string=True)
        addr = int(gdb.selected_frame().read_var("ptr"))
        alloc_desc.addr = addr
        active_allocs[addr] = alloc_desc
        return False

class FreeBreakpoint (gdb.Breakpoint):
    def stop(self):
        #size = gdb.parse_and_eval ('layout.size()') * -1
        addr = int(gdb.selected_frame().read_var("pv")) #cast(gdb.lookup_type("int")).string()
        if addr in active_allocs:
            alloc_desc = active_allocs.pop(addr)
            alloc_desc.dealloc_backtrace = gdb.execute("bt", to_string=True)
            freed_allocs.append(alloc_desc)
        else:
            alloc_desc = AllocationDescriptor()
            alloc_desc.addr = addr
            alloc_desc.dealloc_backtrace = gdb.execute("bt", to_string=True)
            orphan_frees.append(alloc_desc)
        return False

#trace = open("alloc_trace.csv", "w")

# Set up the breakpoint and associate the handler function
#breakpoint = gdb.Breakpoint("freertos_std::sys::freertos::alloc::<impl core::alloc::global::GlobalAlloc for freertos_std::alloc::System>::alloc")
alloc_breakpoint = AllocBreakpoint("debugger_malloc_check",  internal=False)
# Add custom actions to the breakpoint using the 'commands' command
#alloc_breakpoint.commands = \
#    "python alloc_breakpoint_handler()\n"\
#    "continue"
#alloc_breakpoint.silent = True

free_breakpoint = FreeBreakpoint("rust_std_vPortFree",  internal=False)
# Add custom actions to the breakpoint using the 'commands' command
#free_breakpoint.commands = \
#    "python free_breakpoint_handler()\n"\
#    "continue"
#free_breakpoint.silent = True

def write_allocs_json(path_prefix = ""):
    import json

    with open(path_prefix + "active_allocs.json", "w") as file_handle:
        json.dump(active_allocs, file_handle, default=lambda o: o.to_json(), indent=2)

    with open(path_prefix + "freed_allocs.json", "w") as file_handle:
        json.dump(freed_allocs, file_handle, default=lambda o: o.to_json(), indent=2)

    with open(path_prefix + "orphan_frees.json", "w") as file_handle:
        json.dump(orphan_frees, file_handle, default=lambda o: o.to_json(), indent=2)

# Optional: Disable the breakpoint after it's hit once
# breakpoint.silent = True
# breakpoint.continue_after_one = True
