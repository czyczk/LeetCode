package main

import "fmt"

func main() {
	// Expecting 2
	fmt.Println(numWays(2))

	// Expecting 21
	fmt.Println(numWays(7))

	// Expecting 1
	fmt.Println(numWays(0))
}

func numWays(n int) int {
	n += 1
	if n < 2 {
		return n
	}

	a := 0
	b := 1
	for i := 2; i <= n; i++ {
		sum := (a + b) % 1000000007
		a = b
		b = sum
	}

	return b
}
