package part

import (
	"fmt"
	"math/rand"
	"time"
)

// can write in either way
//func add(x, y int) int {
//	return x + y + z
//}

func add(x int, y int) int {
	return x + y
}

func swap(x, y string) (string, string) {
	return y, x
}

func CourseOfGoPartOne() {
	fmt.Println("Starting 1")

	fmt.Println("Hello, World!")
	fmt.Println("The time is", time.Now())
	fmt.Println("My favourite number is:", rand.Intn(10))

	fmt.Println("result of add(1,4) is:", add(1, 4))

	a, b := swap("hello", "world")
	fmt.Println("result of swap(\"hello\", \"world\") is:", a, b)

	var c = 5
	d := 6 // can only be used inside a function
	fmt.Println("c is:", c)
	fmt.Println("d is:", d)

	// numeric constants are high-precision values
	// but you can't print them in Prinf before casting to float64
	const veryBig = 1<<100 + 1.32e+60
	fmt.Printf("%e\n", float64(veryBig))
}
