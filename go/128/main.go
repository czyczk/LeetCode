package main

import (
	"128/hashset"
	"128/unionfind"
)

func main() {
	n1 := []int{100, 4, 200, 1, 3, 2}
	n2 := []int{0, 3, 7, 2, 5, 8, 4, 6, 0, 1}
	n3 := []int{}
	n4 := []int{1, 1, 1}

	// Expecting 4
	println(unionfind.LongestConsecutive(n1))
	println(hashset.LongestConsecutive(n1))
	// Expecting 9
	println(unionfind.LongestConsecutive(n2))
	println(hashset.LongestConsecutive(n2))
	// Expecting 0
	println(unionfind.LongestConsecutive(n3))
	println(hashset.LongestConsecutive(n3))
	// Expecting 1
	println(unionfind.LongestConsecutive(n4))
	println(hashset.LongestConsecutive(n4))
}
