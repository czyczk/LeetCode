package backtracing

import "strconv"

var used []bool
var trace []byte

func ReorderedPowerOf2(n int) bool {
	str := strconv.Itoa(n)
	l := len(str)
	used = make([]bool, l)
	trace = []byte{}

	return backtraceRec([]byte(str))
}

func backtraceRec(str []byte) bool {
	n := len(str)
	lenTrace := len(trace)
	if lenTrace == n {
		num, _ := strconv.Atoi(string(trace))
		return isPowerOf2(num)
	}

	for i := 0; i < n; i++ {
		if used[i] {
			continue
		}

		if lenTrace == 0 && str[i] == byte('0') {
			continue
		}

		used[i] = true

		trace = append(trace, str[i])
		lenTrace++

		ret := backtraceRec(str)
		if ret {
			return true
		}

		trace = trace[:lenTrace-1]
		used[i] = false
		lenTrace--
	}

	return false
}

func isPowerOf2(n int) bool {
	return (n & (n - 1)) == 0
}
