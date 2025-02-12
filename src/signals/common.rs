pub trait Signalable<T> {
    fn get(&self) -> &T;
    fn set(&mut self, val: T) -> ();
    fn subscribe(&mut self, func: fn(val: &T) -> ());
    fn new(val: T) -> Self;
}