use std::alloc::{Layout, alloc, dealloc};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::Add;
use std::ptr::null_mut;
use crate::Class;
use crate::things::{Object, Value};

struct Chunk {
    layout: Layout,
    base: *mut Object,
    curr: *mut Object,
    end: *mut Object,
    size: usize,
}

const MEG: usize = 1 << 20;
const XMX: usize = 16 * MEG;
const XMS: usize = 1 * MEG;

const MIN_CHUNK_SIZE: usize = 1 * MEG;

impl Chunk {
    fn new(size: usize) -> Chunk {
        let layout = Layout::from_size_align(
            size,
            mem::align_of::<Value>()
        ).unwrap();

        let mem;
        unsafe {
            mem = alloc(layout) as *mut Object;
        }
        
        Chunk {
            layout,
            base: mem,
            curr: mem,
            end: unsafe { mem.add(size) },
            size
        }
    }

    unsafe fn remaining_bytes(&self) -> usize {
        (self.base.byte_offset_from(self.curr)) as usize
    }

    fn alloc(&mut self, cls: *const Class) -> *mut Object {
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

    fn size(&self) -> usize {
        self.size
    }

    fn objects(&self) -> Objects {
        Objects {
            current: self.curr,
            end: self.end
        }
    }
}

struct Objects {
    current: *mut Object,
    end: *mut Object,
}

impl Iterator for Objects {
    type Item = *mut Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None
        }
        let ret = self.current;
        let cls = unsafe { &*(&*self.current).class };
        unsafe {
            self.current = self.current
                .add(cls.instance_fields.len() + 1)
        }
        Some(ret)
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe { dealloc(self.base as *mut u8, self.layout) }
    }
}

pub struct Heap {
    bytes_allocated: usize,
    chunks: Vec<Chunk>,
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
            bytes_allocated: chunks.iter().map(Chunk::size).fold(0, usize::add),
            chunks
        }
    }

    fn add_chunk(&mut self, min_size: usize) -> Result<(), ()> {
        let size = max(min_size, MIN_CHUNK_SIZE);
        if self.bytes_allocated + size > XMX {
            return Err(())
        }
        self.chunks.push(Chunk::new(size));
        Ok(())
    }

    unsafe fn garbage_collect(&mut self) -> Vec<*mut Class> {
        let classes_to_collect = vec![];

        fn move_object(to: *mut Object, from: *mut Object) {
            let cls = unsafe { &*((&*from).class) };
            let fields = cls.instance_fields.len();
            for i in 0..=fields {
                unsafe {
                    let from_val = (from as *mut Value).add(i).read();
                    (to as *mut Value).add(i).write(from_val);
                }
            }
        }

        let marked: HashSet<*mut Value> = HashSet::new();
        let mark = |obj: *mut Object| {

        };

        // step one: find all usages and mark them

        for chunk in self.chunks.iter_mut() {
            for obj in chunk.objects() {
              
            }
        }

        // step two: find all objects and kill them, putting rest of heap over old objects

        classes_to_collect
    }
}
