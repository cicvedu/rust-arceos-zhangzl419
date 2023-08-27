//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocError, AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,       //起始位置
    next: usize,        //可用空间起始位置
    allocations: usize, //分配次数
    end: usize,         //全部空间的结尾
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            next: 0,
            allocations: 0,
            end: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.next = _start;
        self.end = _start + _size;
        self.allocations = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let need_size = layout.size();
        let align = layout.align();

        let tmp_start = (self.next + align) / align * align; //对齐内存
        if (tmp_start + need_size) > self.end {
            Err(crate::AllocError::NoMemory)
        } else {
            self.allocations += 1;
            self.next = self.next + need_size;
            Ok(NonZeroUsize::new(tmp_start).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.start = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.start
    }

    fn available_bytes(&self) -> usize {
        self.end - self.next
    }
}
