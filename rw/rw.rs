use std::thread;
use std::sync::{RwLock, Mutex, Arc};
use std::time::Duration;
use std::time::Instant;

fn main() {

    let n = Arc::new(RwLock::new(0));

    let mut handles = vec![];

    //Writer
    for i in 0..10 {
        let n_w = n.clone();
        let write_handle = thread::spawn(move || {

            // let mut n = 0; // Profiling Code
            // let mut ct = Duration::new(0,0); // Profiling Code
            // let mut wt = Duration::new(0,0); // Profiling Code
            // let mut wb; // Profiling Code
            // let mut cb; // Profiling Code
            // let mut ce; // Profiling Code

            // wb = Instant::now(); // Profiling Code
            loop {
            // for i in 0..1000 {
                let mut w = n_w.write().unwrap();
                // cb = Instant::now(); // Profiling Code

                *w += 1;

                // ce = Instant::now(); // Profiling Code
                // ct += ce.duration_since(cb); // Profiling Code
                // wt += cb.duration_since(wb); // Profiling Code

                drop(w);

                // wb = Instant::now(); // Profiling Code
                // n += 1; // Profiling Code
            }

            // println!("Writer {} crit: {:?}", i, ((ct.as_secs() as u32 * 1_000_000_000 + ct.subsec_nanos()) as u32)/n);
            // println!("Writer {} wait: {:?}", i, ((wt.as_secs() as u32 * 1_000_000_000 + wt.subsec_nanos()) as u32)/n);

        });
        handles.push(write_handle);
    }

    //Reader
    for i in 0..10 {
        let n_r = n.clone();
        let read_handle = thread::spawn(move || {

            // let mut n = 0; // Profiling Code
            // let mut ct = Duration::new(0,0); // Profiling Code
            // let mut wt = Duration::new(0,0); // Profiling Code
            // let mut wb; // Profiling Code
            // let mut cb; // Profiling Code
            // let mut ce; // Profiling Code

            let mut val = 0;

            // wb = Instant::now(); // Profiling Code
            loop {
            // for i in 0..1000 {
                let mut r = n_r.read().unwrap();

                // cb = Instant::now(); // Profiling Code

                println!("reader: {}", *r);
                val = *r;

                // ce = Instant::now(); // Profiling Code
                // ct += ce.duration_since(cb); // Profiling Code
                // wt += cb.duration_since(wb); // Profiling Code

                drop(r);

                // wb = Instant::now(); // Profiling Code
                // n += 1; // Profiling Code
            }

            // println!("Reader {} crit: {:?}", i, ((ct.as_secs() as u32 * 1_000_000_000 + ct.subsec_nanos()) as u32)/n);
            // println!("Reader {} wait: {:?}", i, ((wt.as_secs() as u32 * 1_000_000_000 + wt.subsec_nanos()) as u32)/n);
        });
        handles.push(read_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
