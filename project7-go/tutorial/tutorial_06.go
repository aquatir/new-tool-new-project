package tutorial

import (
	"fmt"
	"sync"
	"time"
)

func say(s string, wg *sync.WaitGroup) {
	defer wg.Done()
	time.Sleep(100 * time.Millisecond)
	fmt.Printf(s)
}

func sum(s []int, c chan int) {
	sum := 0
	for _, v := range s {
		sum += v
	}
	c <- sum // send sum to c
}

func CourseOfGoPartSix() {
	fmt.Println("Starting 5")

	var wg sync.WaitGroup
	wg.Add(1)

	go say("world", &wg)
	fmt.Printf("hello ")
	wg.Wait()
	fmt.Println()
	fmt.Println("****")

	s := []int{7, 2, 8, -9, 4, 0, 1, 1}

	c := make(chan int)
	go sum(s[:len(s)/2], c) // will send to channel the sum of first portion of array
	go sum(s[len(s)/2:], c) // will send to channel the sum of second portion of array

	// this works, because there are 2 values in a channel
	x, y := <-c, <-c // receive from c

	fmt.Println(x, y, x+y)
	fmt.Println("****")

	// buffered channels
	// if you make this channel smaller, e.g. make (chan int, 1)
	// the second call ch <- 2 will panic, because nobody is consuming the value, but you want to push another one
	// if you interleave the call "ch <- 1" and fmt.Println(...) it will work as expected
	ch := make(chan int, 2)
	ch <- 1
	ch <- 2
	fmt.Println(<-ch)
	fmt.Println(<-ch)

	fmt.Println("****")

}
