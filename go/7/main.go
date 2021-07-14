package main

import (
	"math"
)

func main() {
	// Expecting 321
	println(reverse(123))
	// Expecting -321
	println(reverse(-123))
	// Expecting 21
	println(reverse(120))
	// Expecting 0
	println(reverse(0))
	// Expecting 109
	println(reverse(901000))
	// Expecting 0
	println(reverse(-2147483648))
}

func reverse(x int) int {
	ret := 0
	for x != 0 {
		ret *= 10
		ret += x % 10
		if ret > math.MaxInt32 || ret < math.MinInt32 {
			return 0
		}
		x /= 10
	}

	return ret
}
