package main

import (
	"fmt"
	"strings"
)

func main() {
	s1, k1 := "aaabb", 3
	s2, k2 := "ababbc", 2
	s3, k3 := "abcdedghijklmnopqrstuvwxyz", 2

	// Expecting 3
	fmt.Println(longestSubstring(s1, k1))
	// Expecting 5
	fmt.Println(longestSubstring(s2, k2))
	// Expecting 0
	fmt.Println(longestSubstring(s3, k3))
}

func longestSubstring(s string, k int) int {
	n := len(s)
	freqs := make([]int, 26)
	for i := 0; i < n; i++ {
		freqs[toIdx(s[i])]++
	}

	split := 0

	for i, ch := range freqs {
		if ch > 0 && ch < k {
			split = i + 'a'
			break
		}
	}

	if split == 0 {
		return n
	}

	ans := 0
	for _, substring := range strings.Split(s, string(rune(split))) {
		ans = max(ans, longestSubstring(substring, k))
	}

	return ans
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}

func toIdx(ch uint8) int {
	return int(ch - 'a')
}
