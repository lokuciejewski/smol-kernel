use core::intrinsics::{volatile_load, volatile_store};

pub fn memory_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

pub fn memory_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}
