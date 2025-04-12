use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, Mutex},
};

fn main() {
    // Cell is Send, not Sync.
    // Sharing type with interior mutability across threads
    // would cause race conditions / undefined behaviour.
    let cell = Cell::new(0);
    // Move is required.
    std::thread::spawn(move || {
        cell.set(1);
    });
    let cell = RefCell::new(0);
    std::thread::spawn(move || {
        let mut n = cell.borrow_mut();
        *n = 1;
    });

    // Rc is not Send or Sync.
    // Interior mutability is required for reference counting (cannot be Sync).
    // Moving a clone of Rc to another thread would violate the reference count.
    let rc = Rc::new(0);
    std::thread::spawn(|| println!("{}", *rc));
    std::thread::spawn(move || println!("{}", *rc));

    // Arc is not Send if T is not Sync.
    let arc = Arc::<Cell<u32>>::new(0.into());
    std::thread::spawn(move || println!("{}", arc.get()));

    // Mutex is Send and Sync if T is Send.
    let cell = Cell::new("123".into());
    let mutex = Mutex::new(cell);
    let arc = Arc::<Mutex<Cell<String>>>::new(mutex);
    std::thread::scope(|s| {
        // Arc is Sync.
        s.spawn(|| println!("{}", arc.lock().unwrap().take()));
    });
    // Arc is Send.
    std::thread::spawn(move || println!("{}", arc.lock().unwrap().take()));
}
