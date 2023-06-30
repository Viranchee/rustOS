#![no_std] // No linking standard library
#![no_main] // Disable Rust entry points

#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rust_os::init();
    println!("Hello World{}", "!");
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();
    
    println!("It did not crash");
    loop{}
}

// Called on Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}
