package main
import (
    "fmt"
    "time"
    "strconv"
    )

type Chamber struct {

    h_load chan bool
    o_load chan bool

    h_ready chan bool
    o_ready chan bool
}

func createChamber() *Chamber {
    c:= new(Chamber)
    c.h_load = make(chan bool)
    c.o_load = make(chan bool)
    c.h_ready = make(chan bool)
    c.o_ready = make(chan bool)

    return c
}

func (c* Chamber) hydrogen(id int) {
    select {
    case <-c.h_load:
    }
    c.h_ready <- true
    select {
    case <-c.o_ready:
    }
    fmt.Println("Bonded H:" + strconv.Itoa(id))
    c.h_load <- true
}

func (c* Chamber) oxygen(id int) {
    select {
    case <-c.o_load:
    }
    for i:= 0; i < 2; i++ {
        select {
        case <-c.h_ready:
        }
    }
    c.o_ready <- true
    c.o_ready <- true
    fmt.Println("Bonded O:" + strconv.Itoa(id))
    c.o_load <- true

}

func (c* Chamber) start(n int) {
    fmt.Println("spinning up go routines")

    for i:=0; i<n; i++ {
        go c.oxygen(i);
    }

    for i:=0; i<n*2; i++ {
        go c.hydrogen(i);
    }

    c.h_load <- true
    c.h_load <- true
    c.o_load <- true
}

func main(){

    fmt.Println("start of main")

    d := createChamber()
    d.start(20)

    time.Sleep(time.Second*3)
    fmt.Println("end of main")
}
