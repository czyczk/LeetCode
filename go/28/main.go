package main

import "fmt"

func main() {
	h1, n1 := "hello", "ll"
	h2, n2 := "aaaaa", "bba"
	h3, n3 := "", ""
	h4, n4 := "mississippi", "issip"

	// Expecting 2
	fmt.Println(strStr(h1, n1))
	// Expecting -1
	fmt.Println(strStr(h2, n2))
	// Expecting 0
	fmt.Println(strStr(h3, n3))
	// Expecting 4
	fmt.Println(strStr(h4, n4))
}

func strStr(haystack string, needle string) int {
	m := len(haystack)
	n := len(needle)

	if n == 0 {
		return 0
	}

	if m == 0 {
		return -1
	}

	next := buildNext(needle)

	j := 0
	for i := 0; i < m; i++ {
		for j > 0 && haystack[i] != needle[j] {
			j = next[j-1]
		}

		if haystack[i] == needle[j] {
			j++
			if j >= n {
				return i - n + 1
			}
		}
	}

	return -1
}

func buildNext(needle string) []int {
	n := len(needle)
	next := make([]int, n)

	j := 0
	for i := 1; i < n; i++ {
		for j > 0 && needle[i] != needle[j] {
			j = next[j-1]
		}

		if needle[i] == needle[j] {
			j++
		}

		next[i] = j
	}

	return next
}
