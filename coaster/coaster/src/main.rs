extern crate std_semaphore;

use std::thread;
use std::sync::{Mutex, Arc};
use std_semaphore::Semaphore;
use std::time::Duration;

const CAP :i32 = 5;

fn main() {

    let n_p = 100;

    let mut handles = vec![];

    let board_queue   = Arc::new(Semaphore::new(0));
    let unboard_queue = Arc::new(Semaphore::new(0));
    let load_queue  = Arc::new(Semaphore::new(0));
    let unload_queue = Arc::new(Semaphore::new(0));

    // Create Passenger Threads.
    for i in 0..n_p {

        let l_q = load_queue.clone();
        let b_q = board_queue.clone();
        let ul_q = unload_queue.clone();
        let ub_q = unboard_queue.clone();

        let p_handle = thread::spawn(move || {

            l_q.acquire();

            println!("Passenger: {} boarded", i);

            b_q.release();

            ul_q.acquire();

            println!("Passenger: {} unboarded", i);

            ub_q.release();

        });
        handles.push(p_handle);
    }

    // Create Roller Coaster Threads.
    for _i in 0..1 {

        let b_q = board_queue.clone();
        let ub_q = unboard_queue.clone();
        let l_q = load_queue.clone();
        let ul_q = unload_queue.clone();

        thread::spawn(move || {

            loop {
                println!("Rollercoaster Loading");
                for _i in 0..CAP {
                    l_q.release();
                }

                for _i in 0..CAP {
                    b_q.acquire();
                }

                println!("Rollercoaster Running");
                thread::sleep(Duration::from_millis(100));
                println!("Rollercoaster Unloading");

                for _i in 0..CAP {
                    ul_q.release();
                }

                for _i in 0..CAP {
                    ub_q.acquire();
                }

                thread::sleep(Duration::from_millis(1000));
            }

        });
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("done");
}
