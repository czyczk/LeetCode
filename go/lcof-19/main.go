package main

import "fmt"

func main() {
	fmt.Println("vim-go")
}

func isMatch(s, p string) bool {
	m, n := len(s), len(p)

	f := make([][]bool, m+1)
	for i := 0; i <= m; i++ {
		f[i] = make([]bool, n+1)
	}

	for i := 0; i <= m; i++ {
		for j := 0; j <= n; j++ {
			if j == 0 || p[j] == '.' {
				f[i][j] = true
				continue
			}

			if p[j] == '*' {
				// Special case for '*':
				//   1: The current string char s[i] matches the pattern char before '*' p[j-1]:
				//      Totally depends on the the result of the last matching attemp. They might be:
				//      1.1: s[i-1] vs. p[j-2]
				//      1.2: s[i] vs. p[j-2]
			}
		}
	}
}
