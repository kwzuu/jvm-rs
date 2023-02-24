use std::alloc::{alloc, dealloc, Layout, LayoutError};
use std::mem;
use std::mem::size_of;
use std::ptr::{addr_of, addr_of_mut, null_mut};
use crate::class::{Class, JavaClass};
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
    fn call(
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
}

pub struct StackFrame {
    // the frame this one is on top of
    under: *mut StackFrame,
    // where we would put the next frame
    above: *mut StackFrame,

    // execution context
    pub program_counter: u32,
    pub method: *const JavaMethod,
    pub class: *mut JavaClass,

    // operand stack and locals
    pub max_stack_and_locals: u32,
    stack_ptr: u32,
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

        for i in 0..max_stack_and_locals as usize {
            frame.stack_and_locals.as_mut_ptr().add(i).write(Value::NULL)
        }
    }
}