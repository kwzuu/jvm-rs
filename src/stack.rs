use std::alloc::{alloc, Layout, LayoutError};
use std::fmt::{Debug, Formatter};

use std::mem::size_of;
use std::ptr::{null_mut};
use std::slice;
use crate::class::{JavaClass};
use crate::method::JavaMethod;
use crate::values::Value;

// there is likely a better implementation than this, something something premature optimization

const MEG: usize = 1 << 20;
const XSS: usize = 4 * MEG;

const LAYOUT: Result<Layout, LayoutError> = Layout::from_size_align(XSS, 8);

pub struct Stack {
    bottom: *mut StackFrame,
    current: *mut StackFrame,
    top: *mut StackFrame,
}

impl Stack {
    pub(crate) unsafe fn new() -> Self {
        let bottom = alloc(LAYOUT.unwrap());
        dbg!(bottom as usize);
        Self {
            bottom: bottom as *mut StackFrame,
            current: bottom as *mut StackFrame,
            top: (bottom.byte_add(XSS)) as *mut StackFrame,
        }
    }

    /// pop a frame off the stack and returns the one under it
    /// old frame is valid until a new call happens
    pub fn ret(&mut self) -> *mut StackFrame {
        let old = self.current;
        unsafe { self.current = (&*old).under; }
        self.current
    }

    /// push, initialize and return a new frame
    pub fn call(
        &mut self,
        method: *const JavaMethod,
        class: *mut JavaClass
    ) -> *mut StackFrame {
        let old = unsafe { &mut *self.current };
        // check for stack overflow
        if old.above > self.top {
            null_mut()
        } else {
            self.current = old.above;
            unsafe { StackFrame::initialize_at(
                self.current,
                old as *mut StackFrame,
                method,
                class
            ) }
            self.current
        }
    }

    pub fn main(
        &mut self,
        method: *const JavaMethod,
        class: *mut JavaClass,
    ) -> *mut StackFrame {
        // TODO: we assume that nobody sets XSS smaller than the main method. this is not a good assumption to make.
        unsafe { StackFrame::initialize_at(self.current, null_mut(), method, class); }
        self.current
    }
}

#[repr(C)]
pub struct StackFrame {
    // the frame this one is on top of
    under: *mut StackFrame,
    // where we would put the next frame
    above: *mut StackFrame,

    // execution context
    pub method: *const JavaMethod,
    pub class: *mut JavaClass,

    // operand stack and locals
    pub max_stack_and_locals: u32,
    stack_ptr: u32,
    pub program_counter: u16,

    stack_and_locals: [Value; 0],
}

impl StackFrame {
    /// initialize a new stack frame at the address
    pub unsafe fn initialize_at(
        // address to put the frame at
        addr: *mut StackFrame,
        // address for the current frame
        under: *mut StackFrame,
        // method the frame is being generated for
        method: *const JavaMethod,
        // class the method belongs to
        class: *mut JavaClass,
    ) {
        let meth = &*method;
        let code = meth.code.as_ref().expect("called abstract method");

        let max_stack_and_locals = code.max_stack as u32 + code.max_locals as u32;

        let frame = &mut *addr;

        // initialize references to frame above and below
        let stride = size_of::<Self>() + max_stack_and_locals as usize * size_of::<Value>();
        frame.under = under;
        frame.above = under.byte_add(stride);

        // initialize execution context for method
        frame.program_counter = 0;
        frame.method = method;
        frame.class = class;

        // initialize stack and locals
        frame.max_stack_and_locals = max_stack_and_locals;
        frame.stack_ptr = code.max_locals as u32;
    }

    #[inline(never)]
    pub fn push(&mut self, value: Value) {
        unsafe {
            let ptr = self.stack_and_locals.as_mut_ptr().add(self.stack_ptr as usize);
            ptr.write(value);
        }

        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_ptr -= 1;
        unsafe {
            self.stack_and_locals.as_mut_ptr()
                .add(self.stack_ptr as usize)
                .read()
        }
    }

    pub fn get(&mut self, n: u16) -> Value {
        unsafe {
            self.stack_and_locals.as_mut_ptr()
                .add(n as usize)
                .read()
        }
    }

    pub fn set(&mut self, n: u16, value: Value) {
        unsafe {
            let vars = self.stack_and_locals.as_mut_ptr();
            let ptr = vars.add(n as usize);
            ptr.write(value);
        }
    }

    pub fn stack_and_locals(&self) -> &[Value] {
        unsafe {
            slice::from_raw_parts(
                self.stack_and_locals.as_ptr(),
                self.stack_ptr as usize
            ) as &[Value]
        }
    }
}

impl Debug for StackFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StackFrame")
            .field("under", &(self.under as usize))
            .field("above", &(self.above as usize))
            .field("method", unsafe { &(&*self.method).name })
            .field("class", unsafe { &(&*self.class).name })
            .field("stack", &self.stack_and_locals())
            .finish()
    }
}