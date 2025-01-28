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
    vga::clear();
    vga::set_background(vga::ColorBase::Green);
    println!("Hello, World!")
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    use core::hint::spin_loop;

    let normal_text = vga::VgaAttr {
        foreground: vga::Color {
            base: vga::ColorBase::Red,
            variant: vga::ColorVariant::Bright,
        },
        background: vga::ColorBase::Black,
        blink: false,
    };
    let highlight = vga::VgaAttr {
        foreground: vga::Color {
            base: vga::ColorBase::White,
            variant: vga::ColorVariant::Dark,
        },
        background: vga::ColorBase::Red,
        blink: false,
    };
    vga::attr(normal_text);
    println!("Kernel panicked!");
    vga::attr(highlight);
    print!("REASON:");
    vga::attr(normal_text);
    println!(" {}", info.message());
    if let Some(location) = info.location() {
        vga::attr(highlight);
        print!("WHERE:");
        vga::attr(normal_text);
        println!(" {}", location);
    }
    loop {
        spin_loop();
    }
}
