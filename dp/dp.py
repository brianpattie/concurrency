import threading
import time


class Philosopher(threading.Thread):

    running = True

    def __init__(self, name, leftFork, rightFork, lefty):
        threading.Thread.__init__(self)
        self.name = name
        self.leftFork = leftFork
        self.rightFork = rightFork
        self.lefty = lefty

    def run(self):
        while self.running:
            print('%s is thinking' % self.name)
            time.sleep(1)
            print('%s is hungry' % self.name)
            self.dine()

    def dine(self):
        fork1 = self.leftFork
        fork2 = self.rightFork

        if self.lefty:
            fork1.acquire(True)
            fork2.acquire(True)
        else:
            fork2.acquire(True)
            fork1.acquire(True)

        self.eating()

        fork1.release()
        fork2.release()

    def eating(self):
        print('%s starts eating' % self.name)
        time.sleep(1)
        print('%s finishes eating' % self.name)


def DiningPhilosophers():
    forks = [threading.Lock() for n in range (5)]

    philosophers = []
    philosophers.append(Philosopher('0', forks[0], forks[1], True))
    philosophers.append(Philosopher('1', forks[1], forks[2], False))
    philosophers.append(Philosopher('2', forks[2], forks[3], False))
    philosophers.append(Philosopher('3', forks[3], forks[4], False))
    philosophers.append(Philosopher('4', forks[4], forks[0], False))

    Philosopher.running = True
    for p in philosophers: p.start()
    time.sleep(20)
    Philosopher.running = False
    print('Exiting')


DiningPhilosophers()
