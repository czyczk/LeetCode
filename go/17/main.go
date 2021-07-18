package main

import "fmt"

func main() {
	d1 := "23"
	d2 := ""
	d3 := "2"

	// Expecting ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
	fmt.Printf("%v\n", letterCombinations(d1))
	// Expecting []
	fmt.Printf("%v\n", letterCombinations(d2))
	// Expecting ["a", "b", "c"]
	fmt.Printf("%v\n", letterCombinations(d3))
}

func letterCombinations(digits string) []string {
	if digits == "" {
		return []string{}
	}

	m := map[byte][]byte{
		'2': []byte("abc"),
		'3': []byte("def"),
		'4': []byte("ghi"),
		'5': []byte("jkl"),
		'6': []byte("mno"),
		'7': []byte("pqrs"),
		'8': []byte("tuv"),
		'9': []byte("wxyz"),
	}

	var ret []string

	var stack []StackStuff
	var trace []byte
	stack = append(stack, StackStuff{
		digitIdx: 0,
		mapIdx:   0,
	})

	for len(stack) != 0 {
		lenStack := len(stack)
		ss := &stack[lenStack-1]
		curDigit := digits[ss.digitIdx]
		if ss.mapIdx >= len(m[curDigit]) {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}

			trace = trace[:len(trace)-1]
			continue
		}

		trace = append(trace, m[curDigit][ss.mapIdx])
		ss.mapIdx++

		if len(trace) == len(digits) {
			ret = append(ret, string(trace))
			trace = trace[:len(trace)-1]
		} else {
			stack = append(stack, StackStuff{
				digitIdx: ss.digitIdx + 1,
				mapIdx:   0,
			})
		}
	}

	return ret
}

type StackStuff struct {
	digitIdx int
	mapIdx   int
}
