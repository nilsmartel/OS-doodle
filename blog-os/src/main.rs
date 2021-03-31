#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::WRITER.lock();
    println!("Hello!");
    wait(300000000);
    writer.write_string("\nYou're awake.");
    wait(300000000);

    for b in b"\nYou've been sleeping for a long time" {
        writer.write_byte(*b);
        wait(10000000);
    }
    for b in b"...\n\n" {
        wait(100000000);
        writer.write_byte(*b);
    }

    for b in b"\nIt's no longer safe here.\n\nWe must go now." {
        writer.write_byte(*b);
        wait(20000000);
    }

    loop {}
}

fn wait(mut cycles: usize) {
    while cycles != 0 {
        cycles -= 1;
    }
}
