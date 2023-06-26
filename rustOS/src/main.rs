#![no_std] // No linking standard library
#![no_main] // Disable Rust entry points

use core::panic::PanicInfo;

// Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

static HELLO: &[u8] = b"Hello World!";

// Don't change name symbol of this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    
    loop{}
}

mod vga_buffer;