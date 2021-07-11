package main

import "fmt"

func main() {
	w1 := []string{"bella", "label", "roller"}
	w2 := []string{"cool", "lock", "cook"}

	// Expecting ["e", "l", "l"]
	fmt.Printf("%v\n", commonChars(w1))
	// Expecting ["c", "o"]
	fmt.Printf("%v\n", commonChars(w2))
}

func commonChars(words []string) []string {
	n := len(words)
	patterns := make([][26]int, n)

	for i, word := range words {
		for j := 0; j < len(word); j++ {
			patterns[i][ch2Idx(word[j])]++
		}
	}

	var result []string
	for i := 0; i < 26; i++ {
		minOccurence := 101
		for _, pattern := range patterns {
			minOccurence = min(minOccurence, pattern[i])
		}

		for j := 0; j < minOccurence; j++ {
			result = append(result, string(idx2Ch(i)))
		}
	}

	return result
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}

func ch2Idx(ch byte) int {
	return int(ch - 'a')
}

func idx2Ch(idx int) byte {
	return byte(idx + 'a')
}
