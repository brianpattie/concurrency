import threading
import time

class Reader(threading.Thread):

    running = True

    def __init__(self, num, turnstile, counter, roomempty, book):
        threading.Thread.__init__(self)
        self.num = num
        self.turnstile = turnstile
        self.counter = counter
        self.roomempty = roomempty
        self.book = book
        self.val = -1

    def run(self):

        n = 0
        ct = 0
        wt = 0

        wb = int(round(time.time() * 1000));
        while self.running:
            self.turnstile.acquire(True)
            self.turnstile.release()

            self.counter.lock.acquire(True)
            self.counter.inc()
            if self.counter.val == 1:
                self.roomempty.acquire()
            self.counter.lock.release()
            cb = int(round(time.time() * 1000));

            #Read start
            self.val = self.book.val
            # print('Reader ' + str(self.num) + ' read ' + str(self.book.val))
            #Read end
            ce = int(round(time.time() * 1000));
            ct += ce - cb
            wt += cb - wb

            self.counter.lock.acquire(True)
            self.counter.dec()
            if self.counter.val == 0:
                self.roomempty.release()
            self.counter.lock.release()
            wb = int(round(time.time() * 1000));
            n += 1

        print('Reader ' + str(self.num) + ' Crit ' + str(ct / n))
        print('Reader ' + str(self.num) + ' Wait ' + str(wt / n))


class Writer(threading.Thread):

    running = True

    def __init__(self, num, turnstile, roomempty, book):
        threading.Thread.__init__(self)
        self.num = num
        self.turnstile = turnstile
        self.roomempty = roomempty
        self.book = book

    def run(self):
        # print('Writer ' + str(self.num) + ' running')
        n = 0
        ct = 0
        wt = 0

        wb = int(round(time.time() * 1000));
        while self.running:
            self.turnstile.acquire(True)

            self.roomempty.acquire(True)
            cb = int(round(time.time() * 1000));

            self.book.inc()
            # print('Writer ' + str(self.num) + ' wrote ' + str(self.book.val))
            ce = int(round(time.time() * 1000));

            ct += ce - cb
            wt += cb - wb

            self.turnstile.release()
            self.roomempty.release()
            wb = int(round(time.time() * 1000));
            n += 1

        print('Writer ' + str(self.num) + ' Crit ' + str(ct / n))
        print('Writer ' + str(self.num) + ' Wait ' + str(wt / n))


class Counter():

    def __init__(self, startval):
        self.lock = threading.Lock()
        self.val = startval

    def inc(self):
        self.val += 1

    def dec(self):
        self.val -= 1


def ReaderWriter():

    turnstile = threading.Lock()
    counter = Counter(0)
    roomempty = threading.Semaphore(1)
    book = Counter(0)

    threads = []
    for i in range(0,10):
        threads.append(Writer(i, turnstile, roomempty, book))
    for i in range(0,10):
        threads.append(Reader(i, turnstile, counter, roomempty, book))

    Reader.running = True
    Writer.running = True
    for t in threads: t.start()
    time.sleep(3)
    Reader.running = False
    Writer.running = False
    print('Exiting')


ReaderWriter()
