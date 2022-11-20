#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(lord_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lord_os::println;

// By using the #[no_mangle] attribute, we disable name mangling to ensure that the Rust compiler really outputs a function with the name _start. Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE symbol to give every function a unique name. The attribute is required because we need to tell the name of the entry point function to the linker in the next step.
// We also have to mark the function as extern "C" to tell the compiler that it should use the C calling convention for this function (instead of the unspecified Rust calling convention). The reason for naming the function _start is that this is the default entry point name for most systems.

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function named `_start` by default.

    println!("Hello world{}", "!");

    lord_os::init();

    // create a breakpoint exception
    //x86_64::instructions::interrupts::int3();

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }

    // trigger a stack overflow
    // fn stack_overflow() {
    //     stack_overflow() // the return address is pushed for each recursion
    // }
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lord_os::test_panic_handler(info)
}

// TESTING
// #[test_case]
// fn trivial_assertion() {
//     assert_eq!(1, 1);
// }
