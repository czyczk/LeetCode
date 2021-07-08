package main

func main() {
	s1, t1 := "anagram", "nagaram"
	s2, t2 := "rat", "car"

	// Expecting true
	println(isAnagram(s1, t1))
	// Expecting false
	println(isAnagram(s2, t2))
}

func isAnagram(s, t string) bool {
	m := len(s)
	n := len(t)

	if m != n {
		return false
	}

	patternS := make([]int, 26)
	patternT := make([]int, 26)

	for i := 0; i < m; i++ {
		patternS[ch2Idx(s[i])]++
	}

	for i := 0; i < n; i++ {
		idx := ch2Idx(t[i])
		patternT[idx]++
		if patternT[idx] > patternS[idx] {
			return false
		}
	}

	return true
}

func ch2Idx(ch byte) int {
	return int(ch - 'a')
}
