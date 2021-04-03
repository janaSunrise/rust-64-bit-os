#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_64_bit_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to my x86_64 bit OS!");

    rust_64_bit_os::init();

    println!("It did not crash!");

    loop {
        use rust_64_bit_os::print;
        print!("-");
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
