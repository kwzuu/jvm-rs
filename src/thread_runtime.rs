/*

use crate::bcw::BidirectionalChannelWrapper;
use crate::class::Class;
use crate::thread::Thread;

pub struct ThreadRuntime {
    parent_runtime: Option<*mut ThreadRuntime>,
    parent_thread: *mut Thread,
    search_channel: BidirectionalChannelWrapper<*mut ()>,
    _cache: Hashmap<String, *mut Class>
}

impl ThreadRuntime {
    fn new() -> Self {
        todo!()
    }

    fn get_class(&mut self, classpath: &str) -> *mut Class {
        if let Some(class) = self._cache.get(classpath) {
            return class;
        } else {
            self.search_channel.send(classpath.as_ptr() as *const (), 0);
            let parent_id = unsafe { &*self.parent_thread }.get_id();

            match self.search_channel.recv() {
                Ok(c_ptr) => {
                    if c_ptr.is_null() {
                        self.thread_panic(
                            &format!(
                                "[THREAD:{}] Class {} not found",
                                parent_id,
                                classpath
                            )
                        );
                    }
                    let c_ptr = c_ptr as *mut Class;
                    self._cache.insert(classpath.to_string(), c_ptr);
                    return c_ptr;
                }
                Err(err) => {
                    self.thread_panic(
                        &format!(
                            "[THREAD:{}] Error while searching for class {}: {}",
                            parent_id,
                            classpath,
                            err
                        )
                    );
                }
            }
        }
    }

    fn thread_panic(&mut self, msg: &str) -> ! {
        todo!()
    }
}
*/
