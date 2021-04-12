package main

import "fmt"

func main() {
	s := "We are happy."
	// We%20are%20happy.
	fmt.Println(replaceSpace(s))
}

func replaceSpace(s string) string {
	n := len(s)
	var result []byte
	for i := 0; i < n; i++ {
		ch := s[i]
		switch ch {
		case ' ':
			result = append(result, '%')
			result = append(result, '2')
			result = append(result, '0')
		default:
			result = append(result, ch)
		}
	}

	return string(result)
}
