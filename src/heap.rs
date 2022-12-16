use std::alloc::{Layout, alloc, dealloc};
use std::cmp::min;
use std::ffi::c_void;
use std::mem;
use std::ptr::null_mut;
use crate::Class;
use crate::things::Value;

struct Chunk<'a> {
    base: *mut Value<'a>,
    curr: *mut Value<'a>,
    end: *mut Value<'a>,
}

const MEG: usize = 1 << 20;
const XMX: usize = 16 * MEG;
const XMS: usize = 1 * MEG;

impl<'a> Chunk<'a> {
    fn new(size: usize) -> Chunk<'a> {
        let layout = Layout::from_size_align(
            size,
            mem::align_of::<Value>()
        ).unwrap();

        unsafe {
            let mem = alloc(layout);
        }
        
        Chunk {
            base: mem,
            curr: mem,
            end: mem + size
        }
    }

    unsafe fn remaining_bytes(&self) -> usize {
        (self.base.byte_offset_from(self.curr)) as usize
    }

    fn alloc(&mut self, cls: *const class) -> *mut Value {
        unsafe {
            let size = cls.instance_fields.len();
        }

        if size > unsafe { self.remaining_bytes() } {
            null_mut()
        } else {
            let p = self.curr;
            self.curr += size;
            unsafe {
                *(p as *mut *const Class) = cls;
            }
            p
        }
    }
}

pub struct Heap<'a> {
    chunks: Vec<Chunk<'a>>
}

impl<'a> Heap<'a> {
    fn new() -> Heap<'a> {
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