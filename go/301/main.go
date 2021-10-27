package main

import "fmt"

func main() {
	s1 := "()())()"
	s2 := "(a)())()"
	s3 := ")("
	s4 := ")t))()()bo)"

	// Expecting ["(())()", "()()()"]
	fmt.Printf("%v\n", removeInvalidParentheses(s1))
	// Expecting ["(a())()", "(a)()()"]
	fmt.Printf("%v\n", removeInvalidParentheses(s2))
	// Expecting [""]
	fmt.Printf("%v\n", removeInvalidParentheses(s3))
	// Expecting ["t(()bo)", "t()(b0)", "t()()bo"]
	fmt.Printf("%v\n", removeInvalidParentheses(s4))
}

var trace []byte
var minNumRemovals int
var ans map[string]bool

func removeInvalidParentheses(s string) []string {
	trace = []byte{}
	minNumRemovals = 26 // more than max len of s
	ans = make(map[string]bool)
	ans[""] = true

	bytes := []byte(s)
	backtraceRec(bytes, 0)

	ret := make([]string, 0, len(ans))
	for k := range ans {
		ret = append(ret, k)
	}
	return ret
}

func backtraceRec(s []byte, offset int) {
	lenS := len(s)
	if lenS == 0 {
		if offset != 0 {
			return
		}

		numRemovals := lenS - len(trace)
		traceStr := string(trace)
		if numRemovals == minNumRemovals {
			ans[traceStr] = true
		} else if numRemovals < minNumRemovals {
			minNumRemovals = numRemovals
			for k := range ans {
				delete(ans, k)
			}
			ans[traceStr] = true
		}

		return
	}

	ch := s[0]

	if ch == byte('(') || ch == byte(')') {
		// Ignore it
		backtraceRec(s[1:], offset)
	}

	// Use it
	trace = append(trace, ch)
	traceStr := string(trace)

	if ch == byte('(') {
		offset++
	} else if ch == byte(')') {
		offset--
	}

	if offset == -1 {
		// Prune
		trace = trace[:len(trace)-1]
		return
	} else if offset == 0 {
		numRemovals := lenS - len(trace)
		if numRemovals == minNumRemovals {
			ans[traceStr] = true
		} else if numRemovals < minNumRemovals {
			minNumRemovals = numRemovals
			for k := range ans {
				delete(ans, k)
			}
			ans[traceStr] = true
		}
	}

	backtraceRec(s[1:], offset)

	trace = trace[:len(trace)-1]
}
