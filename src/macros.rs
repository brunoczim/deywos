#[macro_export]
macro_rules! print {
    ($($tok:tt)*) => {{
        use core::fmt::Write as _;

        $crate::vga::flush();
        let _ = write!($crate::vga::VgaWriter::lock(), $($tok)*);
        $crate::vga::flush();
    }};
}
