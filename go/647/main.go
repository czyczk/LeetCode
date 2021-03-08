package main

import "fmt"

func main() {
	s1 := "abc"
	s2 := "aaa"

	// Expecting 3
	fmt.Println(countSubstrings(s1))
	// Expecting 6
	fmt.Println(countSubstrings(s2))
}

func countSubstrings(s string) int {
	n := len(s)
	t := "$#"
	for i := 0; i < n; i++ {
		t += string(s[i]) + "#"
	}
	t += "!"
	n = len(t)

	f := make([]int, n)

	iMax, rMax, ret := 0, 0, 0
	for i := 1; i < n-1; i++ {
		if i <= rMax {
			f[i] = min(f[2*iMax-i], rMax-i+1)
		} else {
			f[i] = 1
		}

		for t[i+f[i]] == t[i-f[i]] {
			f[i]++
		}

		if i+f[i]-1 > rMax {
			iMax = i
			rMax = i + f[i] - 1
		}

		// ret += upper bound of (f[i] - 1) / 2
		ret += f[i] / 2
	}

	return ret
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
