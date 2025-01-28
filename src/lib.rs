#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;

#[macro_use]
mod macros;

mod libc;
mod spin;
mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    print!("Hello, world!")
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
