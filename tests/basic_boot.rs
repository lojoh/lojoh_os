#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lord_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lord_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function named `_start` by default.

    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lord_os::test_panic_handler(info)
}

// Test cases
#[test_case]
fn test_println() {
    println!("test_println output");
}

// Some ideas for possible future tests are:

// CPU Exceptions: When the code performs invalid operations (e.g.,
// divides by zero), the CPU throws an exception.
// The kernel can register handler functions for such exceptions.
// An integration test could verify that the correct exception handler
// is called when a CPU exception occurs or that the execution
// continues correctly after a resolvable exception.

// Page Tables: Page tables define which memory regions are valid
// and accessible. By modifying the page tables, it is possible to
// allocate new memory regions, for example when launching programs.
// An integration test could modify the page tables in the _start
// function and verify that the modifications have the desired effects
// in #[test_case] functions.

// Userspace Programs: Userspace programs are programs with limited
// access to the system’s resources. For example, they don’t have
// access to kernel data structures or to the memory of other
// programs. An integration test could launch userspace programs that
// perform forbidden operations and verify that the kernel prevents
// them all.
