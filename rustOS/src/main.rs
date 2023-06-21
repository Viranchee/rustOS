#![no_std] // No linking standard library
#![no_main] // Disable Rust entry points

use core::panic::PanicInfo;

// Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// Don't change name symbol of this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Entry point. Linker looks for _start()
    loop{}
}
