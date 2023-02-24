use std::alloc::{Layout, alloc, dealloc};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Add;
use std::ptr::{null_mut};
use crate::class::Class;

use crate::JavaClass;

use crate::values::{Array, Object, Value};

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
        (self.curr.byte_offset_from(self.base)) as usize
    }

    fn alloc(&mut self, cls: *const JavaClass) -> *mut Object {
        let field_count= unsafe { (&*cls).instance_fields.len() };

        if field_count > unsafe { self.remaining_bytes() } {
            null_mut()
        } else {
            let p = self.curr;
            unsafe {
                self.curr = self.curr.add(field_count);
                *(p as *mut *const JavaClass) = cls;
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

    fn rewrite_references(&mut self, _moves: &HashMap<*mut Object, *mut Object>) {

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
                .add(cls.instance_fields().count() + 1)
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
    pub fn new() -> Heap {
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

    /// This is the big one!
    ///
    /// # Arguments
    ///
    /// * `roots`: an iterator over the roots of the runtime
    ///
    /// returns: HashSet<*const Class, Global>
    /// the classes still used by the runtime
    /// any classes non-present can be safely KILLED and MURDERED
    unsafe fn garbage_collect(&mut self, roots: &mut dyn Iterator<Item=*mut Object>) -> HashSet<*const Class> {
        let mut classes_to_save = HashSet::new();

        fn add_class(cls: *const Class, classes_to_save: &mut HashSet<*const Class>) {
            unsafe {
                classes_to_save.insert(cls);
                let super_class = (&*cls).super_class();
                if super_class != null_mut() {
                    add_class(super_class, classes_to_save);
                }
                for interface in (&*cls).interfaces() {
                    add_class(*interface, classes_to_save);
                }
            }
        }

        // step one: find all usages and mark them
        let mut marked: HashSet<*mut Object> = HashSet::new();
        // let mark_array = |arr: *mut Array| {
        //     marked.insert(RefType { arr });
        // };

        fn mark_object(
            obj: *mut Object,
            marked: &mut HashSet<*mut Object>,
            classes_to_save: &mut HashSet<*const Class>
        ) {
            marked.insert(obj);
            unsafe {
                let cls = &*((&*obj).class);
                add_class((&*obj).class, classes_to_save);
                for (_, field) in cls.instance_fields() {
                    if field.is_object() {
                        mark_object(field.get_instance(obj).object(), marked, classes_to_save)
                    }
                }
            }
        }

        for obj in roots {
            mark_object(obj, &mut marked, &mut classes_to_save);
        }

        // step two: find all objects and kill them, putting rest of heap over old objects
        // borks on arrays currently >:3
        let mut moves: HashMap<*mut Object, *mut Object> = HashMap::new();

        fn size_of(obj: *mut Object) -> usize {
            let cls = unsafe { &*((&*obj).class) };
            cls.instance_fields().count() + 1
        }

        // returns how many words to skip to find the next object
        let mut move_object = |from: *mut Object, to: *mut Object| -> usize {
            moves.insert(from, to);
            let size = size_of(from);
            for i in 0..size {
                let from_val = (from as *mut Value).add(i).read();
                (to as *mut Value).add(i).write(from_val);
            }
            size
        };


        for chunk in self.chunks.iter_mut() {
            let mut dest = chunk.base;
            let mut src = chunk.base;

            while src < chunk.curr {
                if marked.contains(&src) {
                    let n = move_object(src, dest);
                    dest = dest.add(n);
                    src = src.add(n);
                } else {
                    src = src.add(size_of(src))
                }
            }

            chunk.curr = dest
        }


        // step three: rewrite all references to new locations
        for chunk in self.chunks.iter_mut() {
            chunk.rewrite_references(&mut moves);
        }

        classes_to_save
    }
}

union RefType {
    obj: *mut Object,
    arr: *mut Array,
}

impl Hash for RefType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (unsafe { self.arr } as usize).hash(state)
    }
}

impl PartialEq<Self> for RefType {
    fn eq(&self, other: &Self) -> bool {
        (unsafe { self.arr } as usize) == (unsafe { other.arr } as usize)
    }
}

impl Eq for RefType {}