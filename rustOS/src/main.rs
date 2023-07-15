#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{bootinfo, entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{hlt_loop, memory::translate_addr, println};
use x86_64::structures::paging::PageTable;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        0xb8000,                          // Identity mapped vga buffer page
        0x201008,                         // some code page
        0x0100_0020_1a10,                 // some stack page
        boot_info.physical_memory_offset, // virtual address to phys page 0
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     println!("Hello World{}", "!");
//     rust_os::init();

//     use x86_64::registers::control::Cr3;
//     let (level_4_page_table, _) = Cr3::read();
//     println!("L4 pt at {:?}", level_4_page_table.start_address());

//     #[cfg(test)]
//     test_main();

//     println!("It did not crash!");
//     rust_os::hlt_loop();
// }

// Called on Panic
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

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
