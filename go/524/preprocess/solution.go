package preprocess

/**
 * Len of s = m,
 * Len of dictionary = n,
 * Len of longest word in dictionary = l,
 * Time = m ^ 2 + n * l
 */
func FindLongestWord(s string, dictionary []string) string {
	m := len(s)
	skipMap := make([][26]int, m+1)
	for i := 0; i <= m; i++ {
		for j := 0; j < 26; j++ {
			skipMap[i][j] = -1
		}
	}

	for j := 0; j < m; j++ {
		idx := toIdx(s[j])
		if skipMap[0][idx] != -1 {
			continue
		}

		skipMap[0][idx] = j
	}

	for i := 0; i < m; i++ {
		for j := i + 1; j < m; j++ {
			idx := toIdx(s[j])
			if skipMap[i+1][idx] != -1 {
				continue
			}

			skipMap[i+1][idx] = j
		}
	}

	maxString := ""
	for _, word := range dictionary {
		if len(word) < len(maxString) {
			continue
		}

		if canMatch(s, word, skipMap) {
			maxString = max(maxString, word)
		}
	}

	return maxString
}

func toIdx(ch byte) int {
	return int(ch - byte('a'))
}

func canMatch(pattern, word string, skipMap [][26]int) bool {
	i, j := -1, 0
	for j < len(word) {
		idx := toIdx(word[j])
		i = skipMap[i+1][idx]
		if i == -1 {
			return false
		}

		j++
	}

	return true
}

func max(strA, strB string) string {
	if len(strA) < len(strB) {
		return strB
	}

	if len(strA) > len(strB) {
		return strA
	}

	for i := 0; i < len(strA); i++ {
		if strA[i] < strB[i] {
			return strA
		} else if strA[i] > strB[i] {
			return strB
		}
	}

	return strB
}
