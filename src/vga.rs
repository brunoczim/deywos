use core::{
    fmt,
    ops::{Deref, DerefMut},
    sync::atomic::{Ordering::SeqCst, fence},
};

use crate::spin::{Mutex, MutexGuard};

pub fn clear() {
    VgaWriter::lock().clear();
}

pub fn attr(attr: VgaAttr) {
    VgaWriter::lock().set_attr(attr);
}

pub fn set_background(color: ColorBase) {
    VgaWriter::lock().set_background(color);
}

pub fn set_foreground(color: Color) {
    VgaWriter::lock().set_foreground(color);
}

pub fn set_foreground_base(base: ColorBase) {
    VgaWriter::lock().set_foreground_base(base);
}

pub fn set_foreground_variant(variant: ColorVariant) {
    VgaWriter::lock().set_foreground_variant(variant);
}

pub fn set_blink(blink: bool) {
    VgaWriter::lock().set_blink(blink);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ColorBase {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    White = 7,
}

impl ColorBase {
    pub fn encode(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorVariant {
    Dark = 0,
    Bright = 1,
}

impl ColorVariant {
    pub fn encode(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub base: ColorBase,
    pub variant: ColorVariant,
}

impl Color {
    pub fn encode(self) -> u8 {
        self.base.encode() | (self.variant.encode() << 3)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VgaAttr {
    pub foreground: Color,
    pub background: ColorBase,
    pub blink: bool,
}

impl Default for VgaAttr {
    fn default() -> Self {
        Self::default_const()
    }
}

impl VgaAttr {
    const fn default_const() -> Self {
        Self {
            foreground: Color {
                base: ColorBase::White,
                variant: ColorVariant::Bright,
            },
            background: ColorBase::Black,
            blink: false,
        }
    }

    pub fn encode(self) -> u8 {
        self.foreground.encode()
            | (self.background.encode() << 4)
            | (u8::from(self.blink) << 7)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VgaCell {
    pub char: u8,
    pub attr: VgaAttr,
}

impl VgaCell {
    pub fn encode(self) -> u16 {
        u16::from_le_bytes([self.char, self.attr.encode()])
    }
}

struct VgaBuffer {
    cells: [[u16; Self::WIDTH]; Self::HEIGHT],
}

impl VgaBuffer {
    pub const WIDTH: usize = 80;
    pub const HEIGHT: usize = 25;

    pub fn get_raw(&self, i: usize, j: usize) -> u16 {
        self.cells[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, cell: VgaCell) {
        self.set_raw(i, j, cell.encode());
    }

    pub fn set_raw(&mut self, i: usize, j: usize, raw_cell: u16) {
        self.cells[i][j] = raw_cell;
    }

    pub fn newline(&mut self) {
        for i in 1 .. self.cells.len() {
            for j in 0 .. self.cells[i].len() {
                let raw_cell = self.get_raw(i, j);
                self.set_raw(i - 1, j, raw_cell);
            }
        }
    }

    pub fn clear_row(
        &mut self,
        row: usize,
        row_size: usize,
        background: ColorBase,
    ) {
        let cell = VgaCell {
            char: b' ',
            attr: VgaAttr {
                foreground: Color {
                    base: background,
                    variant: ColorVariant::Dark,
                },
                background,
                blink: false,
            },
        };
        let encoded = cell.encode();
        for j in 0 .. row_size {
            self.set_raw(row, j, encoded);
        }
    }

    pub fn clear(&mut self, background: ColorBase) {
        let cell = VgaCell {
            char: b' ',
            attr: VgaAttr {
                foreground: Color {
                    base: background,
                    variant: ColorVariant::Dark,
                },
                background,
                blink: false,
            },
        };
        let encoded = cell.encode();
        for i in 0 .. self.cells.len() {
            for j in 0 .. self.cells[i].len() {
                self.set_raw(i, j, encoded);
            }
        }
    }

    unsafe fn get<'a>() -> &'a mut Self {
        const BUFFER_ADDR: *mut VgaBuffer = 0xb8000 as *mut VgaBuffer;

        unsafe { &mut *BUFFER_ADDR }
    }
}

pub fn flush() {
    fence(SeqCst);
}

#[derive(Debug)]
pub struct VgaWriter {
    i: usize,
    j: usize,
    attr: VgaAttr,
}

impl VgaWriter {
    const fn new() -> Self {
        Self { i: 0, j: 0, attr: VgaAttr::default_const() }
    }

    pub fn lock<'a>() -> VgaWriterGuard<'a> {
        static VGA_WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter::new());
        VgaWriterGuard::new(VGA_WRITER.lock())
    }

    pub fn set_attr(&mut self, attr: VgaAttr) {
        self.attr = attr;
    }

    pub fn set_foreground(&mut self, color: Color) {
        self.attr.foreground = color;
    }

    pub fn set_foreground_base(&mut self, base: ColorBase) {
        self.attr.foreground.base = base;
    }

    pub fn set_foreground_variant(&mut self, variant: ColorVariant) {
        self.attr.foreground.variant = variant;
    }

    pub fn set_background(&mut self, color: ColorBase) {
        self.attr.background = color;
    }

    pub fn set_blink(&mut self, blink: bool) {
        self.attr.blink = blink;
    }

    pub fn clear(&mut self) {
        unsafe {
            let buffer = VgaBuffer::get();
            buffer.clear(self.attr.background);
        }
        self.i = 0;
        self.j = 0;
    }

    pub fn write_str(&mut self, chars: &str) {
        flush();
        self.write_str_raw(chars);
        flush();
    }

    pub fn write_bytes(&mut self, chars: &[u8]) {
        flush();
        self.write_bytes_raw(chars);
        flush();
    }

    pub fn write(&mut self, char: u8) {
        flush();
        self.write_raw(char);
        flush();
    }

    fn write_str_raw(&mut self, string: &str) {
        self.write_bytes_raw(string.as_bytes());
    }

    fn write_bytes_raw(&mut self, chars: &[u8]) {
        for char in chars {
            self.write_raw(*char);
        }
    }

    fn write_raw(&mut self, char: u8) {
        if char == b'\n' {
            self.newline();
        } else {
            if self.j >= VgaBuffer::WIDTH - 1 {
                self.newline();
            }
            unsafe {
                let buffer = VgaBuffer::get();
                buffer.set(self.i, self.j, VgaCell { char, attr: self.attr });
            }
            self.j += 1;
        }
    }

    fn newline(&mut self) {
        unsafe {
            let buffer = VgaBuffer::get();
            if self.i < VgaBuffer::HEIGHT {
                self.i += 1;
                self.j = 0;
            } else {
                buffer.newline();
                buffer.clear_row(self.i, self.j, self.attr.background);
                self.j = 0;
            }
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_str_raw(string);
        Ok(())
    }
}

#[derive(Debug)]
pub struct VgaWriterGuard<'a> {
    inner: MutexGuard<'a, VgaWriter>,
}

impl<'a> VgaWriterGuard<'a> {
    fn new(inner: MutexGuard<'a, VgaWriter>) -> Self {
        Self { inner }
    }
}

impl<'a> Deref for VgaWriterGuard<'a> {
    type Target = VgaWriter;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for VgaWriterGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
