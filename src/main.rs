#![no_std] // don't link the Rust standard library, our binary isn't going to run on any OS and hence no standard library will be available
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;


// provide definition of _start() which crt0 (C runtime libary) calls
#[no_mangle] // don't mangle the name of this fn, it needs to be named _start
pub extern "C" fn _start() -> ! {
    println!("Hello world");
    
    #[cfg(test)]
    test_main();
    
    loop {}
}

// provide panic implementation now that standard libary is gone
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}