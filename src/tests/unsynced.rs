use crate::signals::{common::Signalable, signals::Signal};

#[test]
pub fn signal_should_run_callbacks() {
    let mut s = Signal::new(100);

    s.subscribe(|_| {
        assert!(true);
    });

    s.subscribe(|a| {
        assert_eq!(*a, 120);
        println!("Second fn ran");
    });
    s.set(120);
}

#[test]
pub fn signal_should_store_correct_val() {
    let mut s = Signal::new(200);

    s.set(100);
    assert_eq!(*s.get(), 100);
    s.set(110);
    assert_eq!(*s.get(), 110);
}
