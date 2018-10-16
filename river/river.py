import threading
import time
import random
from datetime import datetime

class Counter():

    def __init__(self, elves, orcs):
        self.lock = threading.Lock()
        self.elves = elves
        self.orcs = orcs

    def inc_elves(self, n):
        self.elves += n

    def dec_elves(self, n):
        self.elves -= n

    def num_elves(self):
        return self.elves

    def inc_orcs(self, n):
        self.orcs += n

    def dec_orcs(self, n):
        self.orcs -= n

    def num_orcs(self):
        return self.orcs

class Elf(threading.Thread):

    def __init__(self, id, counter, elf_queue, orc_queue, barrier):
        threading.Thread.__init__(self)
        self.id = id
        self.counter = counter
        self.elf_queue = elf_queue
        self.orc_queue = orc_queue
        self.barrier = barrier
        self.captain = False

    def run(self):

        # time.sleep(random.randrange(0,3))

        # arrival = time.time_ns();

        self.counter.lock.acquire()
        self.counter.inc_elves(1)

        if self.counter.num_elves() == 4:
            self.elf_queue.release()
            self.elf_queue.release()
            self.elf_queue.release()
            self.elf_queue.release()
            self.counter.dec_elves(4)
            self.captain = True
        elif (self.counter.num_elves() == 2 and self.counter.num_orcs() >= 2):
            self.elf_queue.release()
            self.elf_queue.release()
            self.orc_queue.release()
            self.orc_queue.release()
            self.counter.dec_elves(2)
            self.counter.dec_orcs(2)
            self.captain = True
        else:
            self.counter.lock.release()

        self.elf_queue.acquire()

        print("Elf:" + str(self.id) + " Boarding")
        self.barrier.wait()

        if self.captain:
            print("Elf:" + str(self.id) + " Rowing")
            self.counter.lock.release()

        # departure = time.time_ns();
        # print(str((departure - arrival)));

class Orc(threading.Thread):

    def __init__(self, id, counter, elf_queue, orc_queue, barrier):
        threading.Thread.__init__(self)
        self.id = id
        self.counter = counter
        self.elf_queue = elf_queue
        self.orc_queue = orc_queue
        self.barrier = barrier
        self.captain = False

    def run(self):

        # time.sleep(random.randrange(0,3))

        # arrival = time.time_ns();

        self.counter.lock.acquire()
        self.counter.inc_orcs(1)

        if self.counter.num_orcs() == 4:
            self.orc_queue.release()
            self.orc_queue.release()
            self.orc_queue.release()
            self.orc_queue.release()
            self.counter.dec_orcs(4)
            self.captain = True
        elif (self.counter.num_orcs() == 2 and self.counter.num_elves() >= 2):
            self.orc_queue.release()
            self.orc_queue.release()
            self.elf_queue.release()
            self.elf_queue.release()
            self.counter.dec_orcs(2)
            self.counter.dec_elves(2)
            self.captain = True
        else:
            self.counter.lock.release()

        self.orc_queue.acquire()

        print("Orc:" + str(self.id) + " Boarding")
        self.barrier.wait()

        if self.captain:
            print("Orc:" + str(self.id) + " Rowing")
            self.counter.lock.release()

        # departure = time.time_ns();
        # print(str((departure - arrival)));


def RiverCrossing():

    counter = Counter(0, 0)
    elf_queue = threading.Semaphore(0)
    orc_queue = threading.Semaphore(0)
    barrier = threading.Barrier(4)

    people = []
    print("Test 1")
    for i in range(50):
        people.append(Elf(i, counter, elf_queue, orc_queue, barrier))
        people.append(Orc(i, counter, elf_queue, orc_queue, barrier))

    for peep in people: peep.start()
    time.sleep(5)

    print('Exiting')


RiverCrossing()
