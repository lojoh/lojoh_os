#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lord_os::{exit_qemu, serial_println, QemuExitCode};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
