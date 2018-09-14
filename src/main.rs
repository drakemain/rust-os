#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate lazy_static;
extern crate volatile;
extern crate spin;
#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;
extern crate uart_16550;

use core::panic::PanicInfo;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..100 {
        println!("Hello World! {}", i);
    }
    
    serial_println!("Hello Host{}", "!");
    
    unsafe { exit_qemu(); }

    loop {}
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

