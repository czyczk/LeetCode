package rec

import "fmt"

var ret []string
var trace []int

func RestoreIpAddresses(s string) []string {
	if s == "" {
		return []string{}
	}

	ret = []string{}
	backtraceRec(s, 0, 2)
	return ret
}

func backtraceRec(s string, startIdx, endIdx int) {
	if startIdx > endIdx {
		return
	}

	num := 0
	isLeadingZero := false
	for i := startIdx; i <= endIdx; i++ {
		if isLeadingZero {
			return
		}

		num *= 10
		num += int(s[i] - '0')
		if num > 255 {
			return
		}
		if num == 0 {
			isLeadingZero = true
		}

		trace = append(trace, num)

		if len(trace) == 4 {
			if i == len(s)-1 {
				ret = append(ret, fmt.Sprintf("%v.%v.%v.%v", trace[0], trace[1], trace[2], trace[3]))
			}
			trace = trace[:len(trace)-1]
			continue
		} else {
			backtraceRec(s, i+1, min(i+3, len(s)-1))
		}

		trace = trace[:len(trace)-1]
	}

}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
