package main

import "fmt"

func main() {
	t1 := [][]string{{"MUC", "LHR"}, {"JFK", "MUC"}, {"SFO", "SJC"}, {"LHR", "SFO"}}
	t2 := [][]string{{"JFK", "SFO"}, {"JFK", "ATL"}, {"SFO", "ATL"}, {"ATL", "JFK"}, {"ATL", "SFO"}}
	t3 := [][]string{{"JFK", "ATL"}, {"ATL", "JFK"}}

	// Expecting ["JFK", "MUC", "LHR", "SFO", "SJC"]
	fmt.Printf("%v\n", findItinerary(t1))
	// Expecting ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
	fmt.Printf("%v\n", findItinerary(t2))
	// Expecting ["JFK", "ATL", "JFK"]
	fmt.Printf("%v\n", findItinerary(t3))
}

var ans []string
var trace []string
var used []bool
var n int

func findItinerary(tickets [][]string) []string {
	n = len(tickets)
	ans = []string{}
	trace = []string{}
	used = make([]bool, n)
	backtraceRec(tickets)
	return ans
}

func backtraceRec(tickets [][]string) {
	if len(trace) == n+1 {
		return
	}

	for i := 0; i < n; i++ {
		if len(trace) == 0 {
			if tickets[i][0] != "JFK" {
				continue
			}
			trace = append(trace, tickets[i][0])
		} else if tickets[i][0] != trace[len(trace)-1] {
			continue
		}

		if used[i] {
			continue
		}

		trace = append(trace, tickets[i][1])
		used[i] = true

		if len(trace) == n+1 {
			if len(ans) == 0 {
				ans = append(ans, trace...)
			} else {
				isBetter := false
				for i := 0; i < n+1; i++ {
					if trace[i] < ans[i] {
						isBetter = true
						break
					} else if trace[i] > ans[i] {
						break
					}
				}
				if isBetter {
					ans = append([]string{}, trace...)
				}
			}
		}

		backtraceRec(tickets)

		trace = trace[:len(trace)-1]
		used[i] = false
		if len(trace) == 1 {
			trace = trace[:len(trace)-1]
		}
	}
}
