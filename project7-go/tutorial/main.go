package main

import (
	"fmt"
	"tutorial/part"
)

func main() {
	fmt.Println("part one — printing, swapping, constants:")
	part.CourseOfGoPartOne()

	fmt.Println("******\npart two — loops, flow control, deferred, functions \n*****")

	part.CourseOfGoPartTwo()

	fmt.Println("******\npart three — pointers, structs, arrays, slices, maps, closures \n*****")

	part.CourseOfGoPartThree()

	fmt.Println("******\npart four — methods & receivers, interfaces, type check & type switch, implementing interfaces:\n*****")
	part.CourseOfGoPartFour()

	fmt.Println("******\npart five — type parameters:\n*****")
	part.CourseOfGoPartFive()

	fmt.Println("******\npart six — goroutines, channels, mutex:\n*****")
	part.CourseOfGoPartSix()

	fmt.Println("*****\n crawler")
	part.ExecuteCrawler()
}
