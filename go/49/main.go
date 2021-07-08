package main

import "fmt"

func main() {
	strs1 := []string{"eat", "tea", "tan", "ate", "nat", "bat"}
	strs2 := []string{""}
	strs3 := []string{"a"}

	fmt.Printf("%v\n", groupAnagrams(strs1))
	fmt.Printf("%v\n", groupAnagrams(strs2))
	fmt.Printf("%v\n", groupAnagrams(strs3))
}

func groupAnagrams(strs []string) [][]string {
	mapByPattern := make(map[[26]int][]string)

	for _, str := range strs {
		var pattern [26]int

		for i := 0; i < len(str); i++ {
			pattern[ch2Idx(str[i])]++
		}

		mapByPattern[pattern] = append(mapByPattern[pattern], str)
	}

	ret := make([][]string, len(mapByPattern))
	i := 0
	for _, group := range mapByPattern {
		ret[i] = group
		i++
	}

	return ret
}

func ch2Idx(ch byte) int {
	return int(ch - 'a')
}
