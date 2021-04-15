package main

import "fmt"

func main() {
	n1 := []int{3, 4, 5, 1, 2}
	n2 := []int{2, 2, 2, 0, 1}
	n3 := []int{1, 3, 5}

	// Expecting 1
	fmt.Println(minArray(n1))

	// Expecting 0
	fmt.Println(minArray(n2))

	// Expecting 1
	fmt.Println(minArray(n3))
}

func minArray(numbers []int) int {
	first := numbers[0]
	last := numbers[0]
	n := len(numbers)
	if n == 1 {
		return last
	}

	for i := 1; i < n; i++ {
		num := numbers[i]
		if num < last {
			return num
		}

		last = num
	}

	return first
}
