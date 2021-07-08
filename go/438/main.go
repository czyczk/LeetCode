package main

import "fmt"

func main() {
	s1, p1 := "cbaebabacd", "abc"
	s2, p2 := "abab", "ab"

	// Expecting [0, 6]
	fmt.Printf("%v\n", findAnagrams(s1, p1))
	// Expecting [0, 1, 2]
	fmt.Printf("%v\n", findAnagrams(s2, p2))
}

func findAnagrams(s, p string) []int {
	var ret []int
	lenS := len(s)
	lenP := len(p)

	if lenS < lenP {
		return ret
	}

	var patternP [26]int
	for i := 0; i < lenP; i++ {
		patternP[ch2Idx(p[i])]++
	}

	l, r := 0, 0
	var patternS [26]int

	for r < lenS {
		patternS[ch2Idx(s[r])]++

		if r-l+1 == lenP {
			if areSame(patternS[:], patternP[:]) {
				ret = append(ret, l)
			}
			patternS[ch2Idx(s[l])]--
			l++
		}

		r++
	}

	return ret
}

func ch2Idx(ch byte) int {
	return int(ch - 'a')
}

func areSame(patternS, patternP []int) bool {
	for i := 0; i < 26; i++ {
		if patternS[i] != patternP[i] {
			return false
		}
	}

	return true
}
