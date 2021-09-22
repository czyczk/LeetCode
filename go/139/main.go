package main

import "strings"

func main() {
	s1, wd1 := "leetcode", []string{"leet", "code"}
	s2, wd2 := "applepenapple", []string{"apple", "pen"}
	s3, wd3 := "catsandog", []string{"cats", "dog", "sand", "and", "cat"}
	s4, wd4 := "c", []string{"cat"}
	s5, wd5 := "dog", []string{"dog"}

	// Expecting true
	println(wordBreak(s1, wd1))
	// Expecting true
	println(wordBreak(s2, wd2))
	// Expecting false
	println(wordBreak(s3, wd3))
	// Expecting false
	println(wordBreak(s4, wd4))
	// Expecting true
	println(wordBreak(s5, wd5))
}

func wordBreak(s string, wordDict []string) bool {
	n := len(s)
	dp := make([]string, n+1)

	for i := 1; i <= n; i++ {
		for _, word := range wordDict {
			if i < len(word) || (i-len(dp[i-len(word)]) < len(word)) {
				continue
			}

			candidate := dp[i-len(word)] + word
			if strings.Index(s, candidate) == 0 && len(candidate) >= len(dp[i]) {
				dp[i] = candidate
			}

			if i == n && candidate == s {
				return true
			}
		}
	}

	return false
}
