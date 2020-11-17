#![no_std] // don't link the Rust standard library, our binary isn't going to run on any OS and hence no standard library will be available
#![no_main]

// provide panic implementation now that standard libary is gone
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World";

// provide definition of _start() which crt0 (C runtime libary) calls
#[no_mangle] // don't mangle the name of this fn, it needs to be named _start
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // cyan
        }
    }

    loop {}
}