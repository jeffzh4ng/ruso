#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;

use core::panic::PanicInfo;

pub trait Testable {
    fn run (&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // \t tab character aligns the "[ok]"s
        serial_print!("{}...\t", core::any::type_name::<T>()); // any::type_name is implemented by compiler
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests.", tests.len());
    for test in tests {
        test.run(); // calling Testable.run instead of the function directly
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)] // each variant represented with 32 bit integer
pub enum QemuExitCode {
    Success = 0x10, // exit codes are arbitrary, as long as they don't clash with default QEMU exit codes
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32); // port-mapped I/O
    }
}

pub fn init() {
    interrupts::init_idt();
}