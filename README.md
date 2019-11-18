# concurrency

This repo contains solutions to a series of toy concurrency problems taken from The Little Book of Semaphores by Allen B. Downey.

The folders' contents are as follows:
1) bfs - A Rust implementation of a multithreaded breadth-first search algorithm. New threads are spawned to explore each new node, after which they terminate.
2) bfs2 - A Rust implementation of a multithreaded breadth-first search algorithm. Multiple worker threads iterate over the FIFO queue of frontier nodes.  They each grab a node from the frontier, explore all edges of that node, add any new unexplored nodes to the frontier, then grab a new node from the frontier.
3) bfsclassic - A Rust implementation of a single threaded breadth-first search algorithm, written for comparison purposes.
4) coaster - A Go solution and a Rust solution to the Rollercoaster problem. Passenger threads must board the Rollercoaster thread, which waits for all Passengers to board before it resumes. The Passengers must wait for the Rollercoaster to stop before they disembark.  All Passengers must disembark before any new Passengers board the Rollercoaster.
5) dp - A Go solution and a Python solution to the Dining Philosophers problem. 5 Philosophers sit around a circular table, alternating between thinking and eating. 5 forks are on the table, with each philospher having one fork to their left and one to their right.  A philosopher must have both forks in order to eat. The system must not deadlock.
5) h2o - a Go solution and a Rust solution to the H2O porblem. Hydrogen and Oxygen threads combine to form H2O. 2 Hyrdrogens and 1 Oxygen must enter the reaction chamber, react, and exit before any new threads can enter the reaction chamber.
6) river - a Rust solution and a Python solution to the River Crossing problem. Orc threads and Elf threads must cross the river in a boat that seats 4. To prevent a fight breaking out when one group outnumbers the other, they must cross in groups of 4 elves, 4 orcs, or 2 of each.
7) rw - a Rust solution and a Python solution to the Reader-Writer problem. Multiple readers must be able to read a resource while only one writer can write at a time.  Readers cannot read while a writer is writing.  The system must not deadlock.
