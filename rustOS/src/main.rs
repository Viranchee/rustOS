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
    // Entry point. Linker looks for _start()
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop{}
}
