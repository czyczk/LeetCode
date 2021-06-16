package main

import (
	"fmt"
	"unsafe"
)

func main() {
	s1 := "the sky is blue"
	s2 := "  hello world  "
	s3 := "a good   example"

	// Expecting "blue is sky the"
	fmt.Println(reverseWords(s1))
	// Expecting "world hello"
	fmt.Println(reverseWords(s2))
	// Expecting "example good a
	fmt.Println(reverseWords(s3))
}

func reverseWords(s string) string {
	bytes := []byte(s)

	idxStart, idxEnd := trimSpaces(bytes)
	bytes = bytes[idxStart:idxEnd]

	newLen := removeDuplicateSpaces(bytes)
	bytes = bytes[0:newLen]

	reverse(bytes)

	// Reverse each word
	reverseEachWord(bytes)

	// return string(bytes)
	return bytes2str(bytes)
}

func trimSpaces(s []byte) (int, int) {
	// Count ending spaces
	oriLen := len(s)
	numEnding := 0
	for oriLen > 0 {
		if s[oriLen-numEnding-1] == ' ' {
			numEnding++
		} else {
			break
		}
	}

	// Count leading spaces
	numLeading := 0
	for _, ch := range s {
		if ch == ' ' {
			numLeading++
		} else {
			break
		}
	}

	return numLeading, oriLen - numEnding
}

func removeDuplicateSpaces(s []byte) int {
	oriLen := len(s)
	slow, fast := 0, 0
	numRemoved := 0
	isSpaceMode := false

	for ; fast < oriLen; fast++ {
		if s[fast] == ' ' {
			if isSpaceMode {
				numRemoved++
				continue
			}

			isSpaceMode = true
		} else {
			isSpaceMode = false
		}

		s[slow] = s[fast]
		slow++
	}

	return oriLen - numRemoved
}

func reverse(s []byte) {
	head, tail := 0, len(s)-1
	for head < tail {
		temp := s[head]
		s[head] = s[tail]
		s[tail] = temp
		head++
		tail--
	}
}

func reverseEachWord(s []byte) {
	lenS := len(s)
	lastI := 0
	for i, ch := range s {
		if ch == ' ' {
			reverse(s[lastI:i])
			lastI = i + 1
		}
	}

	reverse(s[lastI:lenS])
}

func bytes2str(b []byte) string {
	return *(*string)(unsafe.Pointer(&b))
}
