// #![allow(unused, dead_code)]
#![forbid(unused_must_use)]
#![no_std]
#![no_main]

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    (0..100).for_each(|i| print!("{i} "));
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    loop {}
}
