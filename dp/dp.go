package main
import (
    "fmt"
    "time"
    "strconv"
    "math/rand"
    )

type Diner struct {
    free []chan bool
    done []chan bool
    num_phil int
    started chan int
}

func createDiner(num_phil int) *Diner {
    d:= new(Diner)
    d.num_phil = num_phil
    d.free = make([]chan bool, num_phil)
    d.done = make([]chan bool, num_phil)

    d.started = make(chan int, num_phil)

    for i:=0; i<num_phil; i++ {
        d.free[i] = make(chan bool)
        d.done[i] = make(chan bool)
    }

    return d

}

func (d *Diner) philosopher(id int){
    fmt.Println("philosopher started")

    for {

        left := false
        right := false

        select {
        case left = <- d.free[id]:
            break
        default:
            left = false
            break
        }

        if !left {
            continue
        }

        select {
        case right = <- d.free[((id + 1) % d.num_phil)]:
            break
        default:
            right = false
            break
        }

        if !right {
            d.done[id] <- true
            continue
        }

        fmt.Println(strconv.Itoa(id) + " eating")

        d.done[id] <- true
        d.done[((id + 1) % d.num_phil)] <- true

        fmt.Println(strconv.Itoa(id) + " thinking")
        time.Sleep(time.Millisecond*5)
    }
}

func (d* Diner) fork(id int){
    fmt.Println("fork started")

    for {
        d.free[id] <- true
        select {
        case <-d.done[id]:
        }
    }

}

func getRandTime() time.Duration {
    return time.Millisecond*time.Duration(rand.Intn(1000)/1000)
}

func (d* Diner) start() {
    fmt.Println("spinning up go routines")

    go d.fork(0)
    go d.fork(1)
    go d.fork(2)
    go d.fork(3)
    go d.fork(4)

    time.Sleep(time.Millisecond*10)

    go d.philosopher(0)
    go d.philosopher(1)
    go d.philosopher(2)
    go d.philosopher(3)
    go d.philosopher(4)

}

func main(){

    fmt.Println("start of main")

    d := createDiner(5)
    d.start()

    time.Sleep(time.Millisecond*5000)
    // time.Sleep(5)
    fmt.Println("end of main")
}
