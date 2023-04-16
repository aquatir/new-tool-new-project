package main

import (
	"fmt"
	"project7-go/tutorial"
)

func main() {
	fmt.Println("part one — printing, swapping, constants:")
	tutorial.CourseOfGoPartOne()

	fmt.Println("******\npart two — loops, flow control, deferred, functions \n*****")

	tutorial.CourseOfGoPartTwo()

	fmt.Println("******\npart three — pointers, structs, arrays, slices, maps, closures \n*****")

	tutorial.CourseOfGoPartThree()

	fmt.Println("******\npart four — methods & receivers, interfaces, type check & type switch, implementing interfaces:\n*****")
	tutorial.CourseOfGoPartFour()

	fmt.Println("******\npart five — type parameters:\n*****")
	tutorial.CourseOfGoPartFive()
}
