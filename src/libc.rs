#[unsafe(no_mangle)]
pub unsafe fn memset(dest: *mut u8, ch: i32, len: usize) -> *mut u8 {
    for i in 0 .. len {
        unsafe {
            *dest.add(i) = ch as u8;
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe fn memcmp(left: *const u8, right: *const u8, len: usize) -> i32 {
    for i in 0 .. len {
        unsafe {
            let left_byte = left.add(i);
            let right_byte = right.add(i);
            if left_byte < right_byte {
                return -1;
            }
            if left_byte > right_byte {
                return 1;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    for i in 0 .. len {
        unsafe {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}
