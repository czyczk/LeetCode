package main

import (
	"fmt"
)

func main() {
	customers1 := []int{1, 0, 1, 2, 1, 1, 7, 5}
	grumpy1 := []int{0, 1, 0, 1, 0, 1, 0, 1}
	x1 := 3

	customers2 := []int{2, 6, 6, 9}
	grumpy2 := []int{0, 0, 1, 1}
	x2 := 1

	customers3 := []int{3, 8, 8, 7, 1}
	grumpy3 := []int{1, 1, 1, 1, 1}
	x3 := 3

	// Expecting 16
	fmt.Println(maxSatisfied(customers1, grumpy1, x1))
	// Expecting 17
	fmt.Println(maxSatisfied(customers2, grumpy2, x2))
	// Expecting 23
	fmt.Println(maxSatisfied(customers3, grumpy3, x3))
}

func maxSatisfied(customers []int, grumpy []int, x int) int {
	n := len(customers)
	numSatisfied := 0

	maxCustomersKept := 0
	for i := 0; i < x; i++ {
		maxCustomersKept += customers[i] * grumpy[i]
		numSatisfied += customers[i] * (1 - grumpy[i])
	}
	customersKept := maxCustomersKept

	for i := x; i < n; i++ {
		numSatisfied += customers[i] * (1 - grumpy[i])
		candidateUltStart := i + 1 - x
		customersKept += customers[i]*grumpy[i] - customers[candidateUltStart-1]*grumpy[candidateUltStart-1]
		if customersKept > maxCustomersKept {
			maxCustomersKept = customersKept
		}
	}

	return numSatisfied + maxCustomersKept
}
