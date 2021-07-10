use std::sync::{mpsc, Arc, Mutex};
use std::thread;

//create thread and run some code - wait for thread to complete avoid main thread to finish beforehand
#[test]
fn create_thread() {
    //creates new thread
    let handle = thread::spawn(|| {
        print!("Hello World");
    });

    //wait this thread until another thread is complete
    handle.join().unwrap();
}

//use channels - mpsc
#[test]
fn channels() {
    let (ts, tr) = mpsc::channel();
    //if move is not written it fails - I think my assumption as move is useless now is not true
    thread::spawn(move || {
        ts.send(20).unwrap();
    });

    tr.recv().unwrap();
}

//use mutexes
#[test]
fn mutexes() {
    let a = Arc::new(Mutex::new(10));
    let b = Arc::clone(&a);

    let th = thread::spawn(move || {
        *b.lock().unwrap() = 30;
    });
    th.join().unwrap();
    print!("{:?}", a);
}
