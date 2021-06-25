package main

func main() {
	s1, t1 := "ab#c", "ad#c"
	s2, t2 := "ab##", "c#d#"
	s3, t3 := "a##c", "#a#c"
	s4, t4 := "a#c", "b"
	s5, t5 := "y#fo##f", "y#f#o##f"

	// Expecting true
	println(backspaceCompare(s1, t1))
	// Expecting true
	println(backspaceCompare(s2, t2))
	// Expecting true
	println(backspaceCompare(s3, t3))
	// Expecting false
	println(backspaceCompare(s4, t4))
	// Expecting true
	println(backspaceCompare(s5, t5))
}

func backspaceCompare(s string, t string) bool {
	sBackspaced := applyBackspaces([]byte(s))
	tBackspaced := applyBackspaces([]byte(t))

	// Compare the applied strings
	l := len(sBackspaced)
	if l != len(tBackspaced) {
		return false
	}

	for i := 0; i < l; i++ {
		if sBackspaced[i] != tBackspaced[i] {
			return false
		}
	}

	return true
}

func applyBackspaces(s []byte) []byte {
	l := len(s)
	slow, fast := 0, 0

	for fast < l {
		if s[fast] == '#' {
			if slow > 0 {
				slow--
			}
		} else {
			s[slow] = s[fast]
			slow++
		}

		fast++
	}

	return s[0:slow]
}
