package sort_match

import "sort"

/**
 * Len of s = m,
 * Len of dictionary = n,
 * Len of longest word in dictionary = l,
 * Time = (nlogn * l) + n * max(m, l)
 * Not optimal
 */
func FindLongestWord(s string, dictionary []string) string {
	sort.Slice(dictionary, func(i, j int) bool {
		return stringLess(dictionary[i], dictionary[j])
	})

	for _, word := range dictionary {
		if canMatch(s, word) {
			return word
		}
	}

	return ""
}

func stringLess(strA, strB string) bool {
	if len(strA) < len(strB) {
		return false
	}

	if len(strA) > len(strB) {
		return true
	}

	for i := 0; i < len(strA); i++ {
		if strA[i] < strB[i] {
			return true
		} else if strA[i] > strB[i] {
			return false
		}
	}

	return false
}

func canMatch(pattern, word string) bool {
	i, j := 0, 0
	for j < len(word) {
		if i == len(pattern) {
			return false
		}

		if pattern[i] == word[j] {
			i++
			j++
		} else {
			i++
		}
	}

	return true
}
