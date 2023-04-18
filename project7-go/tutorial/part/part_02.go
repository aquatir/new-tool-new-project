package part

import (
	"fmt"
	"math"
	"runtime"
	"time"
)

func Sqrt(x float64) (z float64) {

	z = x
	precision := 0.0001

	for (z*z)-x > precision {
		z -= (z*z - x) / (2 * z)
	}
	return
}

func CourseOfGoPartTwo() {

	fmt.Println("Starting 2")

	sum := 0
	// classical 3-part for-loop
	for i := 0; i < 10; i++ {
		sum += i
	}
	fmt.Println("sum:", sum)

	// while loop
	for sum < 1000 {
		sum += 50
	}
	fmt.Println("sum:", sum)

	if 5 < 10 {
		fmt.Println("5 < 10 is true")
	} else {
		fmt.Println("5 < 10 is false")
	}

	// "If" statement can start with a small block executed before the if. It may define vars
	// you can see them inside the if statement too
	if v := 5 * 10; v < 60 {
		fmt.Println("v value: ", v)
		fmt.Println("5 * 10 < 60 is true")
	} else {
		fmt.Println("v value: ", v)
		fmt.Println("5 * 10 < 60 is false")
	}

	fmt.Println("square root of 54 (custom func) 	is:", Sqrt(54))
	fmt.Println("square root of 54 (lib func) 		is:", math.Sqrt(54))

	switch os := runtime.GOOS; os {
	case "darwin":
		fmt.Println("OS X.")
	case "linux":
		fmt.Println("Linux.")
	default:
		fmt.Printf("%s.\n", os)
	}

	fmt.Println("When's Saturday?")
	switch today := time.Now().Weekday(); time.Saturday {
	case today:
		fmt.Println("Today.")
	case today + 1:
		fmt.Println("Tomorrow.")
	case today + 2:
		fmt.Println("In two days.")
	default:
		fmt.Println("Too far away.")
	}

	callDeferred()
	callLoopWithDeferred()
}

func callDeferred() {
	defer fmt.Println("deferred call in callDeferred()")
	fmt.Println("last call in callDeferred()")
}

// defer works in a stack fashion. Whichever get in defer the last will be spewed out the first
func callLoopWithDeferred() {
	fmt.Println("counting")

	for i := 0; i < 10; i++ {
		defer fmt.Println(i)
	}

	fmt.Println("done")
}
