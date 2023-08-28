//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        self.next = start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let alloc_start = super::align_up(self.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return Err(AllocError::NoMemory),
        };

        if alloc_end > self.heap_end {
            return Err(AllocError::NoMemory);
        }

        self.next = alloc_end;
        self.allocations += 1;

        Ok(NonZeroUsize::new(alloc_start).unwrap())
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        self.heap_end - self.heap_start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.heap_start
    }

    fn available_bytes(&self) -> usize {
        self.heap_end - self.next
    }
}
