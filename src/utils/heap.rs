//! # Heap Allocator
//!
//! The Kernel attaches an allocator only if the `alloc` feature flag is enabled.

#![feature(alloc)]
#![feature(global_allocator)]

use alloc::alloc::Layout;
use alloc_cortex_m::CortexMHeap;
use cortex_m_rt;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub fn init_heap(size: usize) {
    let start = cortex_m_rt::heap_start() as usize;
    unsafe { ALLOCATOR.init(start, size) }
}

// required: define how Out Of Memory (OOM) conditions should be handled
// *if* no other crate has already defined `oom`
#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(x: Layout) -> ! {
    loop {}
    // force restart system maybe?
}
