#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// By using the #[no_mangle] attribute, we disable name mangling to ensure that the Rust compiler really outputs a function with the name _start. Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE symbol to give every function a unique name. The attribute is required because we need to tell the name of the entry point function to the linker in the next step.
// We also have to mark the function as extern "C" to tell the compiler that it should use the C calling convention for this function (instead of the unspecified Rust calling convention). The reason for naming the function _start is that this is the default entry point name for most systems.

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This function is the entry point, since the linker looks for a function named `_start` by default.

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
