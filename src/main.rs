#![no_std] // don't link the Rust standard library, our binary isn't going to run on any OS and hence no standard library will be available
#![no_main]

mod vga_buffer;

// provide panic implementation now that standard libary is gone
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


// provide definition of _start() which crt0 (C runtime libary) calls
#[no_mangle] // don't mangle the name of this fn, it needs to be named _start
pub extern "C" fn _start() -> ! {
    println!("Hello world");

    loop {}
}