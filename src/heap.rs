use std::alloc::{Layout, alloc, dealloc};
use std::cmp::min;
use std::mem;
use std::ptr::null_mut;
use crate::Class;
use crate::things::Value;

struct Chunk {
    layout: Layout,
    base: *mut Value,
    curr: *mut Value,
    end: *mut Value,
}

const MEG: usize = 1 << 20;
const XMX: usize = 16 * MEG;
const XMS: usize = 1 * MEG;

impl Chunk {
    fn new(size: usize) -> Chunk {
        let layout = Layout::from_size_align(
            size,
            mem::align_of::<Value>()
        ).unwrap();

        let mem;
        unsafe {
            mem = alloc(layout) as *mut Value;
        }
        
        Chunk {
            layout,
            base: mem,
            curr: mem,
            end: unsafe { mem.add(size) }
        }
    }

    unsafe fn remaining_bytes(&self) -> usize {
        (self.base.byte_offset_from(self.curr)) as usize
    }

    fn alloc(&mut self, cls: *const Class) -> *mut Value {
        let field_count;
        unsafe {
            field_count = (&*cls).instance_fields.len();
        }

        if field_count > unsafe { self.remaining_bytes() } {
            null_mut()
        } else {
            let p = self.curr;
            unsafe {
                self.curr = self.curr.add(field_count);
                *(p as *mut *const Class) = cls;
            }
            p
        }
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe { dealloc(self.base as *mut u8, self.layout) }
    }
}

pub struct Heap {
    chunks: Vec<Chunk>
}

impl Heap {
    fn new() -> Heap {
        let chunk_size = min(XMS, MEG);
        let chunk_count = XMS / chunk_size;
        let mut chunks = Vec::with_capacity(chunk_count);
        for _ in 0..chunk_count {
            chunks.push(Chunk::new(chunk_size));
        }
        Heap {
            chunks
        }
    }
}