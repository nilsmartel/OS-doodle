// src/main.rs

mod vga_buffer;

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let mut num: i32 = 0;
    loop {

        for i in 0..32 {
            let bit = num & (1<<i) > 0;
            let value = if bit { b'1' } else { b'0' };

            let index = (HELLO.len() as isize + (31-i) as isize + 1)*2;
            unsafe {
                *vga_buffer.offset(index) = value;
                *vga_buffer.offset(index+1) = 0xb;
            }

            num += 1;
        }
    }
}
