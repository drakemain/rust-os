#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate volatile;
extern crate spin;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..100 {
        println!("Hello World! {}", i);
    }
    panic!("This is a panic!");

    loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}