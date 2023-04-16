package tutorial

import "fmt"

func Index[T comparable](s []T, x T) int {
	for i, v := range s {
		// v and x are type T, which has the comparable
		// constraint, so we can use == here.
		if v == x {
			return i
		}
	}
	return -1
}

type Node[T any] struct {
	next *Node[T]
	val  T
}

func (head *Node[T]) reverse() *Node[T] {
	var prev, cur, next *Node[T] = nil, head, nil

	for cur != nil {
		next = cur.next
		cur.next = prev
		prev = cur
		cur = next
	}

	return prev
}

func (head *Node[T]) print() {
	copyOfHead := head
	for copyOfHead != nil {
		fmt.Printf("%d -> ", copyOfHead.val)
		copyOfHead = copyOfHead.next
	}
}

func CourseOfGoPartFive() {
	fmt.Println("Starting 5")

	// Index works on a slice of ints
	si := []int{10, 20, 15, -10}
	fmt.Println(Index(si, 15))

	// Index also works on a slice of strings
	ss := []string{"foo", "bar", "baz"}
	fmt.Println(Index(ss, "hello"))

	head := &Node[int]{val: 1}
	head.next = &Node[int]{val: 2}
	head.next.next = &Node[int]{val: 3}

	head.print()
	fmt.Println()

	newHead := head.reverse()
	newHead.print()
	fmt.Println()
}
