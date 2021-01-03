package main

import "fmt"

func main() {
	n1 := 2
	n2 := 3
	n3 := 4

	// Expecting 1
	fmt.Println(fib(n1))
	// Expecting 2
	fmt.Println(fib(n2))
	// Expecting 3
	fmt.Println(fib(n3))
}

func fib(n int) int {
	if n < 2 {
		return n
	}

	f0 := 0
	f1 := 1
	for i := 2; i <= n; i++ {
		temp := f1
		f1 = f0 + f1
		f0 = temp
	}

	return f1
}
