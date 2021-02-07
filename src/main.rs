#![no_std]
// don't link the Rust standard library, our binary isn't going to run on any OS and hence no standard library will be available
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{allocator, memory::BootInfoFrameAllocator, println, task::{Task, simple_executor::SimpleExecutor}};


entry_point!(kernel_main); // no need to use extern "C" or no_mangle for entry point now...this macro
                           // defines the real lower level _start entry point for us

// provide definition of _start() which crt0 (C runtime libary) calls
// #[no_mangle] // don't mangle the name of this fn, it needs to be named _start
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{VirtAddr};

    println!("Hello world");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}