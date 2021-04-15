package main

import "fmt"

func main() {
	// Expecting 2
	fmt.Println(climbStairs(2))

	// Expecting 3
	fmt.Println(climbStairs(3))
}

func climbStairs(n int) int {
	n += 1
	if n < 2 {
		return n
	}

	a := 0
	b := 1
	for i := 2; i <= n; i++ {
		sum := a + b
		a = b
		b = sum
	}

	return b
}
