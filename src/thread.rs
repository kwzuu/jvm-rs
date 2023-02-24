use std::sync::atomic::{AtomicU64, Ordering};

pub static mut THREAD_COUNT: AtomicU64 = AtomicU64::new(0);

pub struct Thread {
    id: u64, // should autoincrement
}

impl Thread {
    pub fn new() -> Thread {
        unsafe {
            Thread {
                id: THREAD_COUNT.fetch_add(1, Ordering::SeqCst),
            }
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}

// we don't need to do anything special to drop now, but we will later
/*impl Drop for Thread {
    fn drop(&mut self) {

    }
}*/
