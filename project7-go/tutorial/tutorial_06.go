package tutorial

import (
	"fmt"
	"golang.org/x/tour/tree"
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

func fibonacciChanneled(n int, c chan int) {
	x, y := 0, 1
	for i := 0; i < n; i++ {
		c <- x
		x, y = y, x+y
	}
	close(c)
}

func fibonacciSelect(c, quit chan int) {
	x, y := 0, 1
	for {
		select {
		// if new value -> continue
		case c <- x:
			x, y = y, x+y
		// if time to quit -> exit
		case <-quit:
			fmt.Println("quit fibonacciSelect")
			return
		}
	}
}

// Walk walks the tree t sending all values
// from the tree to the channel ch.
func Walk(t *tree.Tree, ch chan int) {
	if t == nil {
		return
	}
	Walk(t.Left, ch)
	ch <- t.Value
	Walk(t.Right, ch)
}

// Same determines whether the trees
// t1 and t2 contain the same values.
func Same(t1, t2 *tree.Tree) bool {
	ch1, ch2 := make(chan int), make(chan int)
	go func() {
		defer close(ch1)
		Walk(t1, ch1)
	}()
	go func() {
		defer close(ch2)
		Walk(t2, ch2)
	}()
	for {
		v1, ok1 := <-ch1
		v2, ok2 := <-ch2
		if ok1 != ok2 || v1 != v2 { // not entirely correct, because the structure of 2 binary trees might be different
			return false
		}
		if !ok1 && !ok2 {
			break
		}
	}
	return true
}

// SafeCounter is safe to use concurrently.
type SafeCounter struct {
	mu sync.Mutex
	v  map[string]int
}

// Inc increments the counter for the given key.
func (c *SafeCounter) Inc(key string) {
	c.mu.Lock()
	// Lock so only one goroutine at a time can access the map c.v.
	c.v[key]++
	c.mu.Unlock()
}

// Value returns the current value of the counter for the given key.
func (c *SafeCounter) Value(key string) int {
	c.mu.Lock()
	// Lock so only one goroutine at a time can access the map c.v.
	defer c.mu.Unlock()
	return c.v[key]
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

	// range on channel listens until the channel is closed. Only the producer should close the channel
	// you may add a buffer to this channel to speed it up but this is not a strict requirement
	fibChannel := make(chan int)
	go fibonacciChanneled(10, fibChannel)

	for val := range fibChannel {
		fmt.Printf("%d", val)
		fmt.Printf(" ")
	}
	value, isNotClosed := <-fibChannel
	fmt.Println()
	fmt.Println("value", value, "isNotClosed", isNotClosed)
	fmt.Println("****")

	// select blocks until one of the cases can run. Useful when consuming multiple channel
	fibSelectChannel := make(chan int)
	quit := make(chan int)
	go func() {
		for i := 0; i < 10; i++ {
			fmt.Printf("%d ", <-fibSelectChannel)
		}
		quit <- 0
	}() // create a function and call it instantly
	fibonacciSelect(fibSelectChannel, quit)
	fmt.Println()
	fmt.Println("****")

	// a default case is run when others are not satisfied
	tick := time.Tick(100 * time.Millisecond)
	boom := time.After(550 * time.Millisecond)
	go func() {
		for {
			select {
			case <-tick:
				fmt.Print("tick.")
			case <-boom:
				fmt.Print("BOOM!")
				return
			default:
				fmt.Printf(".")
				time.Sleep(50 * time.Millisecond)
			}
		}
	}()

	time.Sleep(600 * time.Millisecond)
	fmt.Println()
	fmt.Println("continuing")
	fmt.Println("*****")

	// mutex works in like most other languages
	safeCounter := SafeCounter{v: make(map[string]int)}
	for i := 0; i < 1000; i++ {
		go safeCounter.Inc("somekey")
	}
	fmt.Println("sleeping for a seconds before the safe keys are updated")
	time.Sleep(time.Second)
	fmt.Println(safeCounter.Value("somekey"))

}
