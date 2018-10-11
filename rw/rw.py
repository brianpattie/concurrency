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
        print('Reader ' + str(self.num) + ' running')
        while self.running:
            self.turnstile.acquire(True)
            self.turnstile.release()

            self.counter.lock.acquire(True)
            self.counter.inc()
            if self.counter.val == 1:
                self.roomempty.acquire()
            self.counter.lock.release()

            #Read start
            self.val = self.book.val
            print('Reader ' + str(self.num) + ' read ' + str(self.book.val))
            #Read end

            self.counter.lock.acquire(True)
            self.counter.dec()
            if self.counter.val == 0:
                self.roomempty.release()
            self.counter.lock.release()

        print('Reader ' + str(self.num) + ' ending')

class Writer(threading.Thread):

    running = True

    def __init__(self, num, turnstile, roomempty, book):
        threading.Thread.__init__(self)
        self.num = num
        self.turnstile = turnstile
        self.roomempty = roomempty
        self.book = book

    def run(self):
        print('Writer ' + str(self.num) + ' running')
        while self.running:
            self.turnstile.acquire(True)
            self.roomempty.acquire(True)

            self.book.inc()
            print('Writer ' + str(self.num) + ' wrote ' + str(self.book.val))

            self.turnstile.release()
            self.roomempty.release()

        print('Writer ' + str(self.num) + ' ending')



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
    for i in range(1,11):
        threads.append(Reader(i, turnstile, counter, roomempty, book))
    for i in range(1,3):
        threads.append(Writer(i, turnstile, roomempty, book))

    Reader.running = True
    Writer.running = True
    for t in threads: t.start()
    time.sleep(1)
    Reader.running = False
    Writer.running = False
    print('Exiting')


ReaderWriter()
