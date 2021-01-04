package main

import "fmt"

func main() {
	str1 := "abbxxxxzyy"
	str2 := "abbxxxxzyyy"
	str3 := "aaa"

	// Expecting [[3, 6]]
	fmt.Println(largeGroupPositions(str1))
	// Expecting [[3, 6], [8, 10]]
	fmt.Println(largeGroupPositions(str2))
	// Expecting [[0, 2]]
	fmt.Println(largeGroupPositions(str3))
}

func largeGroupPositions(s string) [][]int {
	result := [][]int{}
	n := len(s)
	if n == 0 {
		return result
	}

	lastChar := s[0]
	cnt := 1
	start := 0
	end := 0

	for i := 1; i < n; i++ {
		curChar := s[i]
		if curChar == lastChar {
			cnt++
			if cnt >= 3 {
				end = i
			}
		} else {
			lastChar = curChar
			cnt = 1
			if end > start {
				result = append(result, []int{start, end})
			}

			start = i
		}
	}

	if cnt >= 3 {
		result = append(result, []int{start, end})
	}

	return result
}
