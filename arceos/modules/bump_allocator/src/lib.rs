#![no_std]

extern crate alloc;
use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///

#[inline]
const fn align_up(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}
#[inline]
const fn align_down(val: usize, align: usize) -> usize {
    (val) & !(align - 1)
}

pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    count: usize,
    byte_pos: usize,
    page_pos: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            count: 0,
            byte_pos: 0,
            page_pos: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Initialize the allocator with a free memory region.
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.byte_pos = start;
        self.page_pos = self.end;
        self.count = 0;
    }

    /// Add a free memory region to the allocator.
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        unimplemented!();
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    /// Allocate memory with the given size (in bytes) and alignment.
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let start = align_up(self.byte_pos, layout.align());
        let next = start + layout.size();
        if next > self.page_pos {
            alloc::alloc::handle_alloc_error(layout);
        } else {
            self.byte_pos = next;
            self.count += 1;
            NonNull::new(start as *mut u8).ok_or(AllocError::NoMemory)
        }
    }

    /// Deallocate memory at the given position, size, and alignment.
    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        self.count -= 1;
        if self.count == 0 {
            self.byte_pos = self.start;
        }
    }

    /// Returns total memory size in bytes.
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    /// Returns allocated memory size in bytes.
    fn used_bytes(&self) -> usize {
        self.byte_pos - self.start
    }

    /// Returns available memory size in bytes.
    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    /// Allocate contiguous memory pages with given count and alignment.
    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        assert_eq!(align_pow2 % PAGE_SIZE, 0);
        let next = align_down(self.page_pos - num_pages * PAGE_SIZE, PAGE_SIZE);
        if next <= self.byte_pos {
            Err(AllocError::NoMemory)
        } else {
            self.page_pos = next;
            Ok(next)
        }
    } 

    /// Deallocate contiguous memory pages with given position and count.
    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        unimplemented!();
    }

    /// Returns the total number of memory pages.
    fn total_pages(&self) -> usize {
        (self.end - self.start) / PAGE_SIZE
    }

    /// Returns the number of allocated memory pages.
    fn used_pages(&self) -> usize {
        (self.end - self.page_pos) / PAGE_SIZE
    }

    /// Returns the number of available memory pages.
    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / PAGE_SIZE
    }
}
