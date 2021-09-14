package main

import (
	"524/preprocess"
	"524/sort_match"
)

func main() {
	s1, d1 := "abpcplea", []string{"ale", "apple", "monkey", "plea"}
	s2, d2 := "abpcplea", []string{"a", "b", "c"}
	s3, d3 := "barfoofoobarthefoobarman", []string{"bar", "foo", "the"}
	s4, d4 := "abpcplea", []string{"ale", "apple", "monkey", "plea", "abpcplaaa", "abpcllllll", "abccclllpppeeaaaa"}

	// Expecting "apple"
	println(sort_match.FindLongestWord(s1, d1))
	println(preprocess.FindLongestWord(s1, d1))
	// Expecting "a"
	println(sort_match.FindLongestWord(s2, d2))
	println(preprocess.FindLongestWord(s2, d2))
	// Expecting "bar"
	println(sort_match.FindLongestWord(s3, d3))
	println(preprocess.FindLongestWord(s3, d3))
	// Expecting "apple"
	println(sort_match.FindLongestWord(s4, d4))
	println(preprocess.FindLongestWord(s4, d4))
}
