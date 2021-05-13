package main

import "fmt"

func main() {
	// Expecting 1024
	fmt.Println(myPow(2.0, 10))

	// Expecting 9.261
	fmt.Println(myPow(2.1, 3))

	// Expecting 0.25
	fmt.Println(myPow(2, -2))

	// Expecting 0
	fmt.Println(myPow(0.00001, 2147483647))
}

func myPow(x float64, n int) float64 {
	if x == 0 {
		return 0
	} else if x == 1 {
		return 1
	} else if x == -1 {
		if n%2 == 0 {
			return 1
		} else {
			return -1
		}
	}

	if n == 0 {
		return 1
	} else if n < 0 {
		x = 1 / x
		n = -n
	}

	ans := x

	for i := 1; i < n; i++ {
		ans *= x
		if ans == 0 {
			return 0
		}
	}

	return ans
}
