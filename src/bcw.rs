use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::broadcast::{Sender, Receiver, self};
/*
pub static mut BCW_ID: AtomicU64 = AtomicU64::new(0);

struct Broadcast<T> {
    from: u64,
    to: u64,
    data: T,
}

pub struct BidirectionalChannelWrapper<T> {
    pub sender: Sender<Broadcast<T>>,
    pub receiver: Receiver<Broadcast<T>>,
    pub id: u64,
}

impl<T> BidirectionalChannelWrapper<T> {
    pub fn new() -> Self {
        let (sender, receiver) = broadcast::channel(100);
        unsafe {
            Self {
                sender,
                receiver,
                id: BCW_ID.fetch_add(1, Ordering::SeqCst),
            }
        }
    }

    fn subscribe(&self) -> BidirectionalChannelWrapper<T> {
        unsafe {
            Self {
                sender: self.sender.clone(),
                receiver: self.receiver.clone(),
                id: BCW_ID.fetch_add(1, Ordering::SeqCst),
            }
        }
    }

    pub fn send(&mut self, msg: T, to: u64) {
        self.sender.send(Broadcast {
            from: self.id,
            to,
            data: msg,
        }).unwrap();
    }

    pub fn recv(&mut self) -> Result<T, broadcast::error::RecvError> {
        while let Ok(msg) = self.receiver.recv() {
            if msg.to == self.id {
                return Ok(msg.data);
            }
        }
        Err(broadcast::error::RecvError::Closed)
    }
}
*/
