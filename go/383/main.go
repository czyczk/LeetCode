package main

func main() {
	r1, m1 := "a", "b"
	r2, m2 := "aa", "ab"
	r3, m3 := "aa", "aab"

	// Expecting false
	println(canConstruct(r1, m1))
	// Expecting false
	println(canConstruct(r2, m2))
	// Expecting true
	println(canConstruct(r3, m3))
}

func canConstruct(ransomNote, magazine string) bool {
	m, n := len(ransomNote), len(magazine)
	if m > n {
		return false
	}

	materials := make([]int, 26)
	required := make([]int, 26)

	for i := 0; i < n; i++ {
		materials[ch2Idx(magazine[i])]++
	}

	for i := 0; i < m; i++ {
		idx := ch2Idx(ransomNote[i])
		required[idx]++
		if required[idx] > materials[idx] {
			return false
		}
	}

	return true
}

func ch2Idx(ch byte) int {
	return int(ch - 'a')
}
