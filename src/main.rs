#![no_std]
// don't link the Rust standard library, our binary isn't going to run on any OS and hence no standard library will be available
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{memory::BootInfoFrameAllocator, println};

entry_point!(kernel_main); // no need to use extern "C" or no_mangle for entry point now...this macro
                           // defines the real lower level _start entry point for us

// provide definition of _start() which crt0 (C runtime libary) calls
// #[no_mangle] // don't mangle the name of this fn, it needs to be named _start
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::mapper::Translate, VirtAddr};

    println!("Hello world");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };


    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
    #[cfg(test)]
    test_main();

    println!("Does not crash");
    rust_os::hlt_loop();
}

// provide panic implementation now that standard libary is gone
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
