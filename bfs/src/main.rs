extern crate std_semaphore;
extern crate queues;

use std::thread;
use std::sync::{RwLock, Mutex, Arc};
use std_semaphore::Semaphore;
use std::time::Duration;
use queues::*;

const START: i32 = 1; // Start Node
const TARGET: i32 = 4; // End Node
const NODES: i32 = 7; // Number of Nodes

fn main() {

    let adj = [
            [0, 1, 0, 1, 0, 1, 0,],
            [1, 0, 1, 0, 0, 0, 0,],
            [0, 1, 0, 1, 0, 0, 0,],
            [1, 0, 1, 0, 0, 0, 1,],
            [0, 0, 0, 0, 0, 0, 1,],
            [1, 0, 0, 0, 0, 0, 1,],
            [0, 0, 0, 1, 1, 1, 0,],
            ];
    let frontier = Arc::new(Mutex::new(Queue::<i32>::new()));
    let visited = Arc::new(RwLock::new(Vec::new())); // Vector of size NODES
    let path = Arc::new(Mutex::new(Vec::new())); // Vector of size NODES

    //Initialize visited/path
    let x = visited.clone();
    x = vec![0; NODES];
    drop(x);
    let y = path.clone();
    y = vec![0; NODES];
    drop(y);

    let mut handles;

    loop {

        handles = vec![];

        let front_main = frontier.clone(); // FIXME Try skipping the clone and just using frontier directly
        let mut f = front_main.lock().unwrap();
        while f.size() != 0 {
            let s = f.remove().unwrap();
            for i in adj[s] {
                if adj[s][i] != 0 {
                    //LOOP BEGIN
                    let front_c = frontier.clone();
                    let vis_c = visited.clone();
                    let path_c = path.clone();

                    let handle = thread::spawn(move || {

                        let source = s; // TODO SET VALUES FOR SOURCE AND DEST, SCRAP x,y
                        let dest = i; // TODO SET VALUES FOR SOURCE AND DEST, SCRAP x,y

                        let mut v = vis.read().unwrap();
                        if v[dest] == false {
                            drop(v);
                            let mut v = vis.write().unwrap();
                            v[dest] = true;
                            drop(v);

                            let mut p = path_c.lock().unwrap();
                            path[dest] = source;
                            drop(p);

                            if dest == TARGET {

                                let mut p = path_c.lock().unwrap();
                                // This prints the path in reverse order
                                let trace = dest;
                                loop{
                                    println!("{}", trace);
                                    trace = path[trace];
                                    if trace == START { break };
                                }
                                // drop(p) // Unnecessary
                            } else {
                                let mut f = front_c.lock().unwrap();
                                f.add(dest);
                            }
                        }
                    }); // Thread End
                    handles.push(handle);
                }
            } // End of Inner Thread Spawn Loop
        } // End of Outer Thread Spawn Loop
        drop(f);
        for handle in handles {
            handle.join().unwrap();
        }
    }   // End of BFS Loop
}
