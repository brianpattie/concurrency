extern crate std_semaphore;

use std::thread;
use std::sync::{Mutex, Arc};
use std_semaphore::Semaphore;
use std::time::Instant;

fn main() {

    let n_e = 100;
    let n_o = 100;

    let mut handles = vec![];

    let elf_count = Arc::new(Mutex::new(0));
    let orc_count = Arc::new(Mutex::new(0));

    let elf_board = Arc::new(Semaphore::new(0));
    let orc_board = Arc::new(Semaphore::new(0));

    let board_count = Arc::new(Mutex::new(0));
    let disembark = Arc::new(Semaphore::new(0));

    // Create Elf Threads
    for i in 0..n_e {

        let e_c = elf_count.clone();
        let o_c = orc_count.clone();
        let e_b = elf_board.clone();
        let o_b = orc_board.clone();
        let b_c = board_count.clone();
        let d   = disembark.clone();

        let e_handle = thread::spawn(move || {

            let arrival = Instant::now();

            let mut elf_c = e_c.lock().unwrap();
            let mut orc_c = o_c.lock().unwrap();

            *elf_c += 1;

            if *elf_c == 1 { // Not enough Elves to travel
                drop(orc_c);
                drop(elf_c);

                e_b.acquire();

                let mut board_c = b_c.lock().unwrap();
                *board_c += 1;

                // println!("Elf:{} Boarding as crew {}", i, *board_c);

                if *board_c == 3 {
                    *board_c = 0;
                    d.release();
                }

            } else if *elf_c < 4 { // Enough elves to go as 2

                if *orc_c < 2 { // Not enough Orcs to go as 2
                    drop(orc_c);
                    drop(elf_c);

                    e_b.acquire();

                    let mut board_c = b_c.lock().unwrap();
                    *board_c += 1;

                    // println!("Elf:{} Boarding as crew {}", i, *board_c);

                    if *board_c == 3 {
                        d.release();
                        *board_c = 0;
                    }

                } else { // Enough Orcs to go as 2
                    e_b.release();
                    o_b.release();
                    o_b.release();
                    *elf_c -= 2;
                    *orc_c -= 2;

                    d.acquire();
                    // println!("Elf:{} Boarding as captain", i);
                    // println!("Elf:{} Rowing", i);
                }

            } else { // Enough Elves to go as 4
                e_b.release();
                e_b.release();
                e_b.release();
                *elf_c -= 4;

                d.acquire();
                // println!("Elf:{} Boarding as captain", i);
                // println!("Elf:{} Rowing", i);

            }

            let elapsed = arrival.elapsed();
            println!("{:?}", ((elapsed.as_secs() as u32 * 1_000_000_000 + elapsed.subsec_nanos()) as u32));

        });
        handles.push(e_handle);
    }

    // Create Orc Threads
    for i in 0..n_o {

        let e_c = elf_count.clone();
        let o_c = orc_count.clone();
        let e_b = elf_board.clone();
        let o_b = orc_board.clone();
        let b_c = board_count.clone();
        let d   = disembark.clone();

        let o_handle = thread::spawn(move || {

            let arrival = Instant::now();

            let mut elf_c = e_c.lock().unwrap();
            let mut orc_c = o_c.lock().unwrap();

            *orc_c += 1;

            if *orc_c == 1 { // Not enough Orcs to travel
                drop(orc_c);
                drop(elf_c);

                o_b.acquire();

                let mut board_c = b_c.lock().unwrap();
                *board_c += 1;

                // println!("Orc:{} Boarding as crew {}", i, *board_c);

                if *board_c == 3 {
                    *board_c = 0;
                    d.release();
                }

            } else if *orc_c < 4 { // Enough Orcs to go as 2

                if *elf_c < 2 { // Not enough Elves to go as 2
                    drop(orc_c);
                    drop(elf_c);

                    o_b.acquire();

                    let mut board_c = b_c.lock().unwrap();
                    *board_c += 1;

                    // println!("Orc:{} Boarding as crew {}", i, *board_c);

                    if *board_c == 3 {
                        *board_c = 0;
                        d.release();
                    }
                } else { // Enough Elves to go as 2
                    o_b.release();
                    e_b.release();
                    e_b.release();
                    *elf_c -= 2;
                    *orc_c -= 2;

                    d.acquire();
                    // println!("Orc:{} Boarding as captain", i);
                    // println!("Orc:{} Rowing", i);
                }

            } else { // Enough Orcs to go as 4
                o_b.release();
                o_b.release();
                o_b.release();
                *orc_c -= 4;

                d.acquire();
                // println!("Orc:{} Boarding as captain", i);
                // println!("Orc:{} Rowing", i);
            }

            let elapsed = arrival.elapsed();
            println!("{:?}", ((elapsed.as_secs() as u32 * 1_000_000_000 + elapsed.subsec_nanos()) as u32));

        });
        handles.push(o_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("done");
}
