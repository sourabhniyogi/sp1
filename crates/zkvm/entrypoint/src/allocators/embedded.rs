use alloc::alloc::GlobalAlloc;
use alloc::alloc::Layout;
use critical_section::RawRestoreState;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
pub static HEAP: EmbeddedAlloc = EmbeddedAlloc;

pub static INNER_HEAP: Heap = Heap::empty();

struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> RawRestoreState {}

    unsafe fn release(_token: RawRestoreState) {}
}

/// Size of reserved region for input values at the top of the heap.
pub const RESERVED_REGION_SIZE: usize = 1024 * 1024 * 512;

pub fn init() {
    extern "C" {
        static _end: u8;
    }
    let heap_pos: usize = unsafe { (&_end) as *const u8 as usize };
    let heap_size: usize = crate::syscalls::MAX_MEMORY - heap_pos - RESERVED_REGION_SIZE;
    unsafe { INNER_HEAP.init(heap_pos, heap_size) };
}

pub fn used() -> usize {
    critical_section::with(|cs| INNER_HEAP.used())
}

pub fn free() -> usize {
    critical_section::with(|cs| INNER_HEAP.free())
}

pub struct EmbeddedAlloc;

unsafe impl GlobalAlloc for EmbeddedAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        INNER_HEAP.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if (ptr as usize) > crate::syscalls::MAX_MEMORY - RESERVED_REGION_SIZE {
            return;
        }
        INNER_HEAP.dealloc(ptr, layout)
    }
}