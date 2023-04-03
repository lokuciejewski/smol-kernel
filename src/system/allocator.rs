use core::{alloc::Layout, ptr::NonNull};

use linked_list_allocator::LockedHeap;

extern "C" {
    static mut __heap_start: u8;
    static mut __heap_end_exclusive: u8;
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() -> usize {
    unsafe {
        let heap_start = &__heap_start as *const u8 as usize;
        let heap_end = &__heap_end_exclusive as *const u8 as usize;
        let heap_size = heap_end - heap_start;
        ALLOCATOR.force_unlock();
        ALLOCATOR
            .lock()
            .init(&mut __heap_start as *mut u8, heap_size);
        ALLOCATOR.force_unlock();
        return heap_size;
    }
}

pub fn get_free_bytes() -> Option<usize> {
    if ALLOCATOR.is_locked() {
        None
    } else {
        let free = ALLOCATOR.lock().free();
        Some(free)
    }
}

#[no_mangle]
unsafe fn __rust_alloc(layout: Layout) -> *mut u8 {
    let ptr = match ALLOCATOR.lock().allocate_first_fit(layout) {
        Ok(ptr) => ptr.as_ptr(),
        Err(_) => panic!("Memory allocation error"),
    };
    ALLOCATOR.force_unlock();
    ptr
}

#[no_mangle]
unsafe fn __rust_dealloc(ptr: *mut u8, layout: Layout) {
    ALLOCATOR
        .lock()
        .deallocate(NonNull::new(ptr).unwrap(), layout);
    ALLOCATOR.force_unlock();
}

#[no_mangle]
unsafe fn __rust_realloc(ptr: *mut u8, layout: Layout) -> *mut u8 {
    let new_ptr = match ALLOCATOR.lock().allocate_first_fit(layout) {
        Ok(new_ptr) => new_ptr.as_ptr(),
        Err(_) => panic!("Memory allocation error"),
    };
    ALLOCATOR.force_unlock();
    ALLOCATOR
        .lock()
        .deallocate(NonNull::new(ptr).unwrap(), layout);
    ALLOCATOR.force_unlock();
    new_ptr
}

#[no_mangle]
unsafe fn __rust_alloc_zeroed(layout: Layout) -> *mut u8 {
    let ptr = match ALLOCATOR.lock().allocate_first_fit(layout) {
        Ok(ptr) => {
            for i in 0..layout.size() {
                *((ptr.addr().get() + i) as *mut u8) = 0u8
            }
            ptr.as_ptr()
        }
        Err(_) => panic!("Memory allocation error"),
    };
    ALLOCATOR.force_unlock();
    ptr
}

#[alloc_error_handler]
#[no_mangle]
fn __rust_alloc_error_handler(_layout: Layout) -> ! {
    panic!("Memory allocation error");
}

#[no_mangle]
fn __rust_alloc_error_handler_should_panic(_layout: Layout) -> ! {
    panic!("Memory allocation error");
}
