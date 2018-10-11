use std::thread;
use std::sync::{RwLock, Mutex, Arc};
use std::time::Duration;

fn main() {

    // let writing = Arc::new(Mutex::new(0));
    // let read_count = Arc::new(Mutex::new(0));

    let n = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    //Writer
    let n_w = n.clone();
    let write_handle = thread::spawn(move || {
        // for i in 1..10 {
        loop {
            let mut w = n_w.write().unwrap();
            *w += 1;
            drop(w);
            // thread::sleep(Duration::from_millis(1))
        }
    });
    handles.push(write_handle);

    //Reader
    let n_r = n.clone();
    let read_handle = thread::spawn(move || {
        // for i in 1..10 {
        loop {
            let mut r = n_r.read().unwrap();
            println!("reader: {}", *r);
            drop(r);
            // thread::sleep(Duration::from_millis(1))
        }
    });
    handles.push(read_handle);

    for handle in handles {
        handle.join().unwrap();
    }



}
