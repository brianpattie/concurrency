extern crate std_semaphore;

use std::thread;
use std::sync::{Mutex, Arc};
use std_semaphore::Semaphore;
use std::time::Duration;

const CAP :i32 = 5;

fn main() {

    let n_p = 100;

    let mut handles = vec![];

    let board_count   = Arc::new(Mutex::new(0));
    let unboard_count = Arc::new(Mutex::new(0));

    let board_queue   = Arc::new(Semaphore::new(0));
    let unboard_queue = Arc::new(Semaphore::new(0));
    let full  = Arc::new(Semaphore::new(0));
    let empty = Arc::new(Semaphore::new(0));

    // Create Passenger Threads.
    for i in 0..n_p {

        let b_c = board_count.clone();
        let ub_c = unboard_count.clone();
        let b_q = board_queue.clone();
        let ub_q = unboard_queue.clone();
        let f = full.clone();
        let e = empty.clone();

        let p_handle = thread::spawn(move || {

            //Boarding
            b_q.acquire();

            let mut board_c = b_c.lock().unwrap();
            println!("Passenger: {} boarded", i);
            *board_c += 1;
            if *board_c == CAP {
                f.release();
                *board_c = 0;
            }
            drop(board_c);

            //Unboarding
            ub_q.acquire();

            let mut unboard_c = ub_c.lock().unwrap();
            println!("Passenger: {} unboarded", i);
            *unboard_c += 1;
            if *unboard_c == CAP {
                e.release();
                *unboard_c = 0;
            }
            drop(unboard_c);

        });
        handles.push(p_handle);
    }

    // Create Roller Coaster Threads.
    for _i in 0..1 {

        let b_q = board_queue.clone();
        let ub_q = unboard_queue.clone();
        let f = full.clone();
        let e = empty.clone();

        /*let r_handle = */thread::spawn(move || {

            loop {
                println!("Rollercoaster Loading");
                for _i in 0..CAP {
                    b_q.release();
                }

                f.acquire();

                println!("Rollercoaster Running");
                thread::sleep(Duration::from_millis(100));
                println!("Rollercoaster Unloading");

                for _i in 0..CAP {
                    ub_q.release();
                }

                e.acquire();

                thread::sleep(Duration::from_millis(1000));
            }

        });
        // handles.push(r_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("done");
}
