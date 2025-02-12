use super::common::Signalable;
// Just wrap the Signal in an Arc<Mutex<Signal<T>>> to use across threads safely.
pub struct Signal<T> {
    val: T,
    callbacks: Vec<fn(val: &T) -> ()>,
}

impl<T> Signalable<T> for Signal<T> {
    fn get(&self) -> &T {
        return &self.val;
    }

    fn set(&mut self, val: T) -> () {
        self.val = val;
        for callback in self.callbacks.iter() {
            callback(&self.val);
        }
    }

    fn subscribe(&mut self, func: fn(val: &T) -> ()) {
        self.callbacks.push(func);
    }
    fn new(val: T) -> Self {
        Self {
            val,
            callbacks: vec![]
        }
    }
}
