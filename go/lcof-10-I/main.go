package main

import "fmt"

func main() {
	// Expecting 1
	fmt.Println(fib(2))
	// Expecting 5
	fmt.Println(fib(5))
	// Expecting 407059028
	fmt.Println(fib(95))
}

func fib(n int) int {
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
