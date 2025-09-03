#![no_std]

#[cfg(not(test))]
use core::panic::PanicInfo;

use crate::vga::{ColorBase, ColorVariant, VgaWriter};

#[macro_use]
mod macros;

mod libc;
pub mod spin;
pub mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    vga::init();
    let mut vga_writer = VgaWriter::lock();
    vga_writer
        .set_foreground_base(ColorBase::Brown)
        .set_foreground_variant(ColorVariant::Bright)
        .set_background(ColorBase::Green);
    plockedln!(vga_writer, "Hello, World!");
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    use core::hint::spin_loop;

    {
        let mut writer = vga::VgaWriter::lock();
        let prev_attr = writer.attr();
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
        writer.set_attr(normal_text);
        plockedln!(writer, "Kernel panicked!");
        writer.set_attr(highlight);
        plocked!(writer, "REASON");
        writer.set_attr(normal_text);
        plockedln!(writer, ": {}", info.message());
        if let Some(location) = info.location() {
            writer.set_attr(highlight);
            plocked!(writer, "WHERE");
            writer.set_attr(normal_text);
            plockedln!(writer, ": {}", location);
        }
        writer.set_attr(prev_attr);
    }
    loop {
        spin_loop();
    }
}
