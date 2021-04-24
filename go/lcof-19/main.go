package main

import "fmt"

func main() {
	s1, p1 := "aa", "a"
	s2, p2 := "aa", "a*"
	s3, p3 := "ab", ".*"
	s4, p4 := "aab", "c*a*b"
	s5, p5 := "aasdfasdfasdfasdfas", "aasdf.*asdf.*asdf.*asdf.*s"

	// Expecting false
	fmt.Println(isMatch(s1, p1))

	// Expecting true
	fmt.Println(isMatch(s2, p2))

	// Expecting true
	fmt.Println(isMatch(s3, p3))

	// Expecting true
	fmt.Println(isMatch(s4, p4))

	// Expecting true
	fmt.Println(isMatch(s5, p5))
}

func isMatch(s, p string) bool {
	m, n := len(s), len(p)

	f := make([][]bool, m+1)
	for i := 0; i <= m; i++ {
		f[i] = make([]bool, n+1)
	}

	f[0][0] = true

	for i := 0; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if p[j-1] == '*' {
				// Special case for '*':
				//   1: The current string char s[i] matches the pattern char before '*' p[j-1]:
				//      Totally depends on the the result of the last matching attemp. They might be:
				//      1.1: s[0..i-1] vs. p[0..j-3]
				//      1.2: s[0..i-2] vs. p[0..j-1]
				if i > 0 && (p[j-2] == '.' || s[i-1] == p[j-2]) {
					f[i][j] = f[i][j-2] || f[i-1][j]
				} else {
					if j >= 2 {
						f[i][j] = f[i][j] || f[i][j-2]
					}
					if i >= 1 && j >= 2 && (s[i-1] == p[j-2] || p[j-2] == '.') {
						f[i][j] = f[i][j] || f[i-1][j]
					}
				}
			} else {
				if i > 0 && (p[j-1] == '.' || s[i-1] == p[j-1]) {
					f[i][j] = f[i-1][j-1]
				}
			}
		}
	}

	return f[m][n]
}
