package main

import "fmt"

func main() {
	s1 := "abab"
	s2 := "aba"
	s3 := "abcabcabc"
	s4 := "abac"
	s5 := "ababba"

	// Expecting true
	fmt.Println(repeatedSubstringPattern(s1))
	// Expecting false
	fmt.Println(repeatedSubstringPattern(s2))
	// Expecting true
	fmt.Println(repeatedSubstringPattern(s3))
	// Expecting false
	fmt.Println(repeatedSubstringPattern(s4))
	// Expecting false
	fmt.Println(repeatedSubstringPattern(s5))
}

func repeatedSubstringPattern(s string) bool {
	next, n := buildNext(s)
	if n == 0 || next[n-1] == 0 {
		return false
	}

	if n%(n-next[n-1]) == 0 {
		return true
	}

	return false
}

func buildNext(s string) ([]int, int) {
	n := len(s)
	next := make([]int, n)

	j := 0
	for i := 1; i < n; i++ {
		for j > 0 && s[i] != s[j] {
			j = next[j-1]
		}

		if s[i] == s[j] {
			j++
		}

		next[i] = j
	}

	return next, n
}
