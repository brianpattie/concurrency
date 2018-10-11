use std::thread;
use std::sync::{Semaphore, Arc};
use std::time::Duration;

struct Table {
    forks: Vec<Semaphore<()>>
}

fn left(i: i32) -> i32 {
    return i;
}

fn right(i: i32) -> i32 {
    return (i + 1) % 5;Semaphore
}

// fn leftie_get_fork() {
//
// }
//
// fn righie_get_fork() {
//
// }
//
// fn drop_fork() {
//
// }

fn main() {

    // let writing = Arc::new(Mutex::new(0));
    // let read_count = Arc::new(Mutex::new(0));

    let table = Arc::new(Table {
        forks: vec![
            Semaphore::new(1),
            Semaphore::new(1),
            Semaphore::new(1),
            Semaphore::new(1),
            Semaphore::new(1),
        ]
    });

    table.forks[0].acquire();
    println!("Locked fork");
    table.forks[0].release();

    // let n = Arc::new(RwLock::new(0));
    //
    // let mut handles = vec![];
    //
    // //Writer
    // let n_w = n.clone();
    // let write_handle = thread::spawn(move || {
    //     // for i in 1..10 {
    //     loop {
    //         let mut w = n_w.write().unwrap();
    //         *w += 1;
    //         drop(w);
    //         // thread::sleep(Duration::from_millis(1))
    //     }
    // });
    // handles.push(write_handle);
    //
    // //Reader
    // let n_r = n.clone();
    // let read_handle = thread::spawn(move || {
    //     // for i in 1..10 {
    //     loop {
    //         let mut r = n_r.read().unwrap();
    //         println!("reader: {}", *r);
    //         drop(r);
    //         // thread::sleep(Duration::from_millis(1))
    //     }
    // });
    // handles.push(read_handle);
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }

}
