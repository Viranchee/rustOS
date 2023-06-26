#![no_std] // No linking standard library
#![no_main] // Disable Rust entry points

use core::panic::PanicInfo;
mod vga_buffer;

// Called on Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

static HELLO: &[u8] = b"Hello World!";

// Don't change name symbol of this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", 3.33);
    panic!("NOPE> DONT CALL THIS");
    loop{}
}
