extern crate std_semaphore;

use std::thread;
use std::sync::Arc;
use std_semaphore::Semaphore;

fn main() {

    let n_o = 10;
    let n_h = n_o*2;

    let mut handles = vec![];

    let h_loader = Arc::new(Semaphore::new(2));
    let o_loader = Arc::new(Semaphore::new(1));
    let h_ready = Arc::new(Semaphore::new(0));
    let o_ready = Arc::new(Semaphore::new(0));

    // Create Hydrogen Threads.
    for i in 0..n_h {

        let h_l = h_loader.clone();
        let h_r = h_ready.clone();
        let o_r = o_ready.clone();

        let h_handle = thread::spawn(move || {

            // Wait for Hydrogen Loader.
            h_l.acquire();

            // Signal to Oxygen in reaction chamber that the Hydrogens are ready.
            h_r.release();

            // Wait for Oxygen to return ready signal
            o_r.acquire();

            println!("Bonded H:{}", i); // Bond

            // Signal next Hydrogen to load
            h_l.release();
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
