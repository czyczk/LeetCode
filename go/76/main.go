package main

func main() {
	s1, t1 := "ADOBECODEBANC", "ABC"
	s2, t2 := "a", "a"
	s3, t3 := "a", "aa"
	s4, t4 := "a", "b"

	// Expecting "BANC"
	println(minWindow(s1, t1))
	// Expecting "a"
	println(minWindow(s2, t2))
	// Expecting ""
	println(minWindow(s3, t3))
	// Expecting ""
	println(minWindow(s4, t4))
}

func minWindow(s string, t string) string {
	if len(s) < len(t) {
		return ""
	}

	numsRequired := make(map[byte]int)
	for i := 0; i < len(t); i++ {
		numsRequired[t[i]]++
	}

	n := len(s)
	numsTaken := make([]int, 128)
	numTypesTaken := 0
	minWindow := n + 1
	bestL := n + 1

	l := 0

greatLoop:
	for r := 0; r < n; r++ {
		for {
			// Expand `r` to reach the first hit
			if numsTaken[s[r]] == 0 {
				numTypesTaken++
			}

			numsTaken[s[r]]++
			if numTypesTaken >= len(numsRequired) {
				allTaken := true
				for k, v := range numsRequired {
					if numsTaken[k] < v {
						allTaken = false
						break
					}
				}

				if allTaken {
					break
				}
			}

			r++
			if r == n {
				break greatLoop
			}
		}

		for l < r {
			// Shrink `l` to get the minimum window
			if numsRequired[s[l]] > 0 && numsTaken[s[l]] == numsRequired[s[l]] {
				break
			}

			if numsTaken[s[l]] == 1 {
				numTypesTaken--
			}

			numsTaken[s[l]]--
			l++
		}

		candidateWindowSize := r - l + 1
		if candidateWindowSize == 1 {
			return string(s[r])
		}

		if candidateWindowSize < minWindow {
			minWindow = candidateWindowSize
			bestL = l
		}

	}

	if minWindow == n+1 {
		return ""
	}

	return string(s[bestL : bestL+minWindow])
}
