#![no_std] // don't link the Rust standard library, our binary isn't going to run on any OS and hence no standard library will be available
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

// provide panic implementation now that standard libary is gone
use core::panic::PanicInfo;

// reg panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// provide definition of _start() which crt0 (C runtime libary) calls
#[no_mangle] // don't mangle the name of this fn, it needs to be named _start
pub extern "C" fn _start() -> ! {
    println!("Hello world");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests.", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
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
        port.write(exit_code as u32);
    }
}

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