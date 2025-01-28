#[macro_export]
macro_rules! print {
    ($($tok:tt)*) => {{
        use core::fmt::Write as _;

        $crate::vga::flush();
        let _ = write!($crate::vga::VgaWriter::lock(), $($tok)*);
        $crate::vga::flush();
    }};
}

#[macro_export]
macro_rules! println {
    ($($tok:tt)*) => {{
        use core::fmt::Write as _;

        $crate::vga::flush();
        let _ = writeln!($crate::vga::VgaWriter::lock(), $($tok)*);
        $crate::vga::flush();
    }};
}

#[macro_export]
macro_rules! plocked {
    ($dest:expr, $($tok:tt)*) => {{
        use core::fmt::Write as _;

        $crate::vga::flush();
        let _ = write!($dest, $($tok)*);
        $crate::vga::flush();
    }};
}

#[macro_export]
macro_rules! plockedln {
    ($dest:expr, $($tok:tt)*) => {{
        use core::fmt::Write as _;

        $crate::vga::flush();
        let _ = writeln!($dest, $($tok)*);
        $crate::vga::flush();
    }};
}
