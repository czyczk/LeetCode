package main

func main() {
	s1 := "RLRRLLRLRL"
	s2 := ""
	s3 := "RLLLLRRRLR"
	s4 := "LLLLRRRR"
	s5 := "RLRRRLLRLL"

	// Expecting 4
	println(balancedStringSplit(s1))
	// Expecting 0
	println(balancedStringSplit(s2))
	// Expecting 3
	println(balancedStringSplit(s3))
	// Expecting 1
	println(balancedStringSplit(s4))
	// Expecting 2
	println(balancedStringSplit(s5))
}

func balancedStringSplit(s string) int {
	n := len(s)
	ret := 0
	offset := 0

	for i := 0; i < n; i++ {
		if s[i] == 'L' {
			offset--
		} else {
			offset++
		}

		if offset == 0 {
			ret++
		}
	}

	return ret
}
