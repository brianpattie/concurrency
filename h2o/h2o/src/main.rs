extern crate std_semaphore;

use std::thread;
use std::sync::{Mutex, Arc};
use std_semaphore::Semaphore;

fn main() {

    let n_o = 10;
    let n_h = n_o*2;

    let mut handles = vec![];

    let h_loader = Arc::new(Semaphore::new(2));
    let o_loader = Arc::new(Semaphore::new(1));
    let h_ready = Arc::new(Semaphore::new(0));
    let o_ready = Arc::new(Semaphore::new(0));
    let h_count = Arc::new(Mutex::new(0));

    // let bonded = Arc::new(Mutex::new(<Vec<i32>>::new()));

    // Create Hydrogen Threads.
    for i in 0..n_h {

        let h_l = h_loader.clone();
        let h_r = h_ready.clone();
        let h_c = h_count.clone();
        let o_r = o_ready.clone();

        let h_handle = thread::spawn(move || {

            // Wait for Hydrogen Loader.
            h_l.acquire();
            // println!("Loaded H:{}", i);

            // Increment counter of hydrogens in reaction chamber.
            let mut count = h_c.lock().unwrap();
            *count = *count + 1;
            drop(count); // Release lock

            // Signal to Oxygen in reaction chamber that the Hydrogens are ready.
            h_r.release();

            // Wait for Oxygen to return ready signal
            o_r.acquire();

            let mut count = h_c.lock().unwrap();

            if *count == 2 { // First Hydrogen to leave.  Decrement counter.
                *count -= 1;
                println!("Bonded H:{}", i);
                drop(count);
            } else if *count == 1{
                *count -= 1; // Both Hydrogens are leaving.  Open the loader for the next 2.
                println!("Bonded H:{}", i);
                drop(count);
                h_l.release();
                h_l.release();
            } else {
                println!("ERROR: h_count is poisoned.");
            }
        });
        handles.push(h_handle);
    }

    // Create Oxygen Threads
    for i in 0..n_o {

        let o_l = o_loader.clone();
        let o_r = o_ready.clone();
        let h_r = h_ready.clone();

        let o_handle = thread::spawn(move || {

            // Wait for Oxygen loader.
            o_l.acquire();
            // println!("Loaded O:{}", i);

            // Wait for Hydrogen to be ready.
            h_r.acquire();
            h_r.acquire();

            // Signal to Hydrogen that Oxygen is also ready.
            o_r.release();
            o_r.release();
            println!("Bonded O:{}", i);

            // Open loader for next Oxygen.
            o_l.release();

        });
        handles.push(o_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("done");
}
