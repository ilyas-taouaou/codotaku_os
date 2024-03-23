#![allow(unused, dead_code)]
#![forbid(unused_must_use)]
#![no_std]
#![no_main]

use core::fmt::Write;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

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
