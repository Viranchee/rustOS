#![no_std] // No linking standard library
#![no_main] // Disable Rust entry points

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;

// Called on Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}



// Don't change name symbol of this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();
    
    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1,1);
    println!("[ok]");
}