package main

import (
	"869/solution/backtracing"
	"869/solution/hashset"
)

func main() {
	// Expecting true
	println(backtracing.ReorderedPowerOf2(1))
	println(hashset.ReorderedPowerOf2(1))
	// Expecting false
	println(backtracing.ReorderedPowerOf2(10))
	println(hashset.ReorderedPowerOf2(10))
	// Expecting true
	println(backtracing.ReorderedPowerOf2(16))
	println(hashset.ReorderedPowerOf2(16))
	// Expecting false
	println(backtracing.ReorderedPowerOf2(24))
	println(hashset.ReorderedPowerOf2(24))
	// Expecting true
	println(backtracing.ReorderedPowerOf2(46))
	println(hashset.ReorderedPowerOf2(46))
	// Expecting true
	println(backtracing.ReorderedPowerOf2(679213508))
	println(hashset.ReorderedPowerOf2(679213508))
}
