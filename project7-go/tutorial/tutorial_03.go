package tutorial

import (
	"fmt"
	"golang.org/x/tour/pic"
	"golang.org/x/tour/wc"
	"math"
	"strings"
)

func printSlice(s []int) {
	fmt.Printf("len=%d cap=%d %v\n", len(s), cap(s), s)
}

func wordCount(s string) map[string]int {
	res := make(map[string]int)
	words := strings.Split(s, " ")
	for _, word := range words {
		res[word] = res[word] + 1
	}
	return res
}

// can accept other functions inside functions
// this one accepts a function that take two floats and return a that always applies the first argument
func saveFirstArg(fn func(float64, float64) float64, first float64) func(float64) float64 {
	return func(second float64) float64 {
		return fn(first, second)
	}
}

func adder() func(int) int {
	sum := 0
	return func(x int) int {
		sum += x
		return sum
	}
}

// fibonacci is a function that returns
// a function that returns an int.
func fibonacci() func() int {
	prev2 := 0
	prev1 := 1
	n := -1
	return func() int {
		n++
		if n < 1 {
			return n
		}
		cur := prev1 + prev2
		prev2 = prev1
		prev1 = cur
		return cur
	}
}

func Pic(dx, dy int) [][]uint8 {

	// create two-dimensional array
	res := make([][]uint8, dy)
	for x := range res {
		res[x] = make([]uint8, dx)
	}

	// add values
	for y := 0; y < dy; y++ {
		for x := 0; x < dx; x++ {
			res[x][y] = uint8(x * y)
		}
	}

	return res
}

// Vertex struct == a collection of fields
type Vertex struct {
	X int
	Y int
}

func CourseOfGoPartThree() {
	fmt.Println("Starting 3")

	var p *int // p is a pointer to int
	i := 42
	p = &i                        // p now points to i
	fmt.Println("i, *p: ", i, *p) // read i through the pointer p
	*p = 21                       // set i through the pointer p
	fmt.Println("i, *p:", i, *p)

	fmt.Println(Vertex{1, 2})

	// access struct fields
	vertex := Vertex{5, 6}
	fmt.Printf("{x, y}: {%d, %d}\n", vertex.X, vertex.Y)

	// can access struct with pointers
	// Go permits the easier form of access
	pointerToVertex := &vertex
	(*pointerToVertex).X = 15
	pointerToVertex.Y = 40
	fmt.Printf("{x, y}: {%d, %d}\n", pointerToVertex.X, pointerToVertex.Y)
	fmt.Printf("{x, y}: {%d, %d}\n", vertex.X, vertex.Y)

	// can use the Name: syntax to define structs
	v1 := Vertex{X: 5, Y: 10}
	v2 := Vertex{X: 1} // Y:0 is implicit
	v3 := Vertex{}     // X:0 and Y:0

	fmt.Println(v1, v2, v3)

	// [n]T is an array of n values of type T
	var arr [10]int
	fmt.Println("array:", arr)

	var vertexArr [2]Vertex
	fmt.Println("vertexArr:", vertexArr)

	// slices
	arrr := [...]int{1, 2, 3, 4, 5}
	fmt.Println("[1:4] slice of", arrr, "is", arrr[1:4])
	fmt.Println("[0:5] slice of", arrr, "is", arrr[0:5])
	fmt.Println("[3:] slice of", arrr, "is", arrr[3:])
	fmt.Println("[:3] slice of", arrr, "is", arrr[:3])
	fmt.Println("[:] slice of", arrr, "is", arrr)

	// slices are like references
	oneThree := arrr[1:4]
	oneThree[0] = 97
	oneThree[1] = 98
	oneThree[2] = 99

	fmt.Println("arrr", arrr, "oneThree", oneThree, "'[1:4]' slice", arrr[1:4])

	// you can have slice literals
	s := []struct {
		i int
		b bool
	}{
		{2, true},
		{3, false},
		{5, true},
		{7, true},
		{11, false},
		{13, true},
	}
	fmt.Println(s)

	slice := []int{2, 3, 5, 7, 11, 13}
	printSlice(slice)

	// Slice the slice to give it zero length.
	slice = slice[:0]
	printSlice(slice)

	// Extend its length.
	slice = slice[:4]
	printSlice(slice)

	// Drop its first two values.
	slice = slice[2:]
	printSlice(slice)

	sliceOfSize5 := make([]int, 5)
	printSlice(sliceOfSize5)

	// append works

	fmt.Println("****")
	var theSlice []int
	theSlice = append(theSlice, 1)
	theSlice = append(theSlice, 2)
	printSlice(theSlice)
	theSlice = append(theSlice, 3, 4, 5, 6)
	printSlice(theSlice)

	// range
	var pow = []int{1, 2, 4, 8, 16, 32, 64, 128}
	for index, value := range pow {
		fmt.Printf("2**%d = %d\n", index, value)
	}

	pic.Show(Pic)

	// maps
	nameToAge := make(map[string]int)
	nameToAge["Ivan"] = 18
	nameToAge["Maria"] = 27
	fmt.Println(nameToAge)

	// map literal
	cityToPopulation := map[string]int{
		"Dubai":  4_000_000,
		"Moscow": 15_000_000,
	}
	fmt.Println(cityToPopulation)

	elem, isPresent := cityToPopulation["Tokyo"]
	fmt.Println("is present", isPresent, "element", elem)

	wc.Test(wordCount)

	twoInPowerOf := saveFirstArg(math.Pow, 2)
	fmt.Println(twoInPowerOf(3))

	pos, neg := adder(), adder()
	for i := 0; i < 10; i++ {
		fmt.Println(
			pos(i),
			neg(-2*i),
		)
	}

	f := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(f())
	}
}
