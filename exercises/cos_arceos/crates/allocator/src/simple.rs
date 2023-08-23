//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator, AllocError};

pub struct SimpleByteAllocator {
    data: [u8; Self::Max],
    // 指向当前可用内存
    curFreeMemPtr: usize,
    // 已分配数
    num_allocated: usize
}

impl SimpleByteAllocator {
    const Max: usize = 10 * 1024 * 1024;
    
    pub const fn new() -> Self {
        Self {
            data: [0; Self::Max],
            curFreeMemPtr: 0,
            num_allocated: 0
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = layout.size();
        let align = 2usize.pow(layout.align() as u32);

        let quotient = size / align;
        let remainder = size % align;
        let size = if remainder != 0 {
            quotient + 1
        } else {
            quotient
        } * align;
        
        //确保内存足够
        if self.curFreeMemPtr + size > Self::Max {
            return Err(AllocError::NoMemory);
        }

        //分配
        let start = self.curFreeMemPtr;
        self.curFreeMemPtr += size;
        self.num_allocated += 1;
        let ptr = self.data[start..self.curFreeMemPtr].as_mut_ptr() as usize;

        Ok(NonZeroUsize::new(ptr).unwrap())
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.num_allocated -= 1;
        if self.num_allocated == 0 {
            self.curFreeMemPtr = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        Self::Max
    }

    fn used_bytes(&self) -> usize {
        self.curFreeMemPtr
    }

    fn available_bytes(&self) -> usize {
        Self::Max - self.curFreeMemPtr
    }
}