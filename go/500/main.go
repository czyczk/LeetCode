package main

import "fmt"

func main() {
	words1 := []string{"Hello", "Alaska", "Dad", "Peace"}
	words2 := []string{"omk"}
	words3 := []string{"adsdf", "sfd"}

	// Expecting ["Alaska", "Dad"]
	fmt.Printf("%v\n", findWords(words1))
	// Expecting []
	fmt.Printf("%v\n", findWords(words2))
	// Expecting ["adsdf", "sfd"]
	fmt.Printf("%v\n", findWords(words3))
}

var keyRowMap map[byte]int

func findWords(words []string) []string {
	var ans []string

wordLoop:
	for _, word := range words {
		lastRow := 0
		for i := 0; i < len(word); i++ {
			if i == 0 {
				lastRow = keyRowMap[word[0]]
				continue
			}

			row := keyRowMap[word[i]]
			if row != lastRow {
				continue wordLoop
			}
		}

		ans = append(ans, word)
	}

	return ans
}

func init() {
	if keyRowMap != nil {
		return
	}

	keyRowMap = make(map[byte]int)

	for _, key := range []byte("qwertyuiopQWERTYUIOP") {
		keyRowMap[key] = 0
	}

	for _, key := range []byte("asdfghjklASDFGHJKL") {
		keyRowMap[key] = 1
	}

	for _, key := range []byte("zxcvbnmZXCVBNM") {
		keyRowMap[key] = 2
	}
}
