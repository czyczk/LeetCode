package main

import "fmt"

func main() {
	s1 := "25525511135"
	s2 := "0000"
	s3 := "1111"
	s4 := "010010"
	s5 := "101023"

	// Expecting ["255.255.11.135", "255.255.111.35"]
	fmt.Printf("%v\n", restoreIpAddresses(s1))
	// Expecting ["0.0.0.0"]
	fmt.Printf("%v\n", restoreIpAddresses(s2))
	// Expecting ["1.1.1.1"]
	fmt.Printf("%v\n", restoreIpAddresses(s3))
	// Expecting ["0.10.0.10", "0.100.1.0"]
	fmt.Printf("%v\n", restoreIpAddresses(s4))
	// Expecting ["1.0.10.23", "1.0.102.3", "10.1.0.23", "10.10.2.3", "101.0.2.3"]
	fmt.Printf("%v\n", restoreIpAddresses(s5))
}

func restoreIpAddresses(s string) []string {
	if s == "" {
		return []string{}
	}

	n := len(s)

	var stack []StackStuff
	{
		stack = append(stack, StackStuff{
			startIdx: 0,
			endIdx:   2,
		})
	}

	var ret []string
	var trace []int
	for len(stack) != 0 {
		lenStack := len(stack)
		ss := &stack[lenStack-1]

		if ss.startIdx > ss.endIdx || ss.isLeadingZero {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}

			trace = trace[:len(trace)-1]
			continue
		}

		ss.num *= 10
		ss.num += int(s[ss.startIdx] - '0')
		if ss.num == 0 {
			ss.isLeadingZero = true
		}

		if ss.num > 255 {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}

			trace = trace[:len(trace)-1]
			continue
		}

		trace = append(trace, ss.num)

		if len(trace) == 4 {
			if ss.startIdx == n-1 {
				ret = append(ret, fmt.Sprintf("%v.%v.%v.%v", trace[0], trace[1], trace[2], trace[3]))
			}

			trace = trace[:len(trace)-1]
		} else {
			newStartIdx := ss.startIdx + 1
			stack = append(stack, StackStuff{
				startIdx: newStartIdx,
				endIdx:   min(newStartIdx+2, n-1),
			})
			ss = &stack[len(stack)-2]
		}

		ss.startIdx++
	}

	return ret
}

type StackStuff struct {
	isLeadingZero bool
	num           int
	startIdx      int
	endIdx        int // inclusive
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
