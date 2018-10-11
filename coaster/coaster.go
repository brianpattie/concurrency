package main
import (
    "fmt"
    "time"
    "strconv"
    )

type Station struct {

    load chan bool
    unload chan bool
    board chan bool
    unboard chan bool
    capacity int

}

func createStation(cap int) *Station {
    s:= new(Station)
    s.load = make(chan bool)
    s.unload = make(chan bool)
    s.board = make(chan bool)
    s.unboard = make(chan bool)
    s.capacity = cap
    return s
}

func (s* Station) passenger(id int) {

    // Wait for Load
    select {
    case <-s.load:
    }

    fmt.Println("Boarded:"  + strconv.Itoa(id))

    // Signal that passenger has boarded
    s.board <- true

    // Wait for unload
    select {
    case <-s.unload:
    }

    fmt.Println("Unboarded:"  + strconv.Itoa(id))

    // Signal that passenger has unboarded
    s.unboard <- true


}

func (s* Station) rollercoaster() {

    for {
        fmt.Println("Rollercoaster Loading")

        // Signal ready for loading
        for i:=0; i<s.capacity; i++ {
            s.load <- true
        }

        // Wait for passengers to board
        for i:=0; i<s.capacity; i++ {
            select {
            case <-s.board:
            }
        }

        fmt.Println("Rollercoaster Running")
        time.Sleep(time.Millisecond*100)
        fmt.Println("Rollercoaster Unloading")

        for i:=0; i<s.capacity; i++ {
            s.unload <- true
        }

        for i:=0; i<s.capacity; i++ {
            select {
            case <-s.unboard:
            }
        }

        time.Sleep(time.Millisecond*1000)
    }
}

func (s* Station) start(n int) {
    fmt.Println("spinning up go routines")

    go s.rollercoaster()

    for i:=0; i<n; i++ {
        go s.passenger(i);
    }
}

func main(){

    fmt.Println("start of main")

    s := createStation(5)
    s.start(20)
    time.Sleep(time.Second*10)

    fmt.Println("end of main")
}
