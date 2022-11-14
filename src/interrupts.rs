use crate::println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// I need to store the IDT where it has 'static lifetime (e.g. it lives for the entire lifetime of the program) but I do not have access to the heap.
// Therefore I use a static mut. Not idiomatic but the best I can come up with for now. Maybe I'll find a better solution later on.
static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
