package main

import "fmt"

func main() {
	b1 := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'C', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	w1 := "ABCCED"

	b2 := [][]byte{
		{'A', 'B'},
		{'C', 'D'},
	}
	w2 := "abcd"

	b3 := [][]byte{{'a'}}
	w3 := "ab"

	b4 := b2
	w4 := "ABCD"

	b5 := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'E', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	w5 := "ABCESEEEFS"

	// Expecting true
	fmt.Println(exist(b1, w1))
	// Expecting false
	fmt.Println(exist(b2, w2))
	// Expecting false
	fmt.Println(exist(b3, w3))
	// Expecting false
	fmt.Println(exist(b4, w4))
	// Expecting true
	fmt.Println(exist(b5, w5))
}

type Status int

const (
	Pending Status = iota
	Visited
)

func exist(board [][]byte, word string) bool {
	n := len(board)
	m := len(board[0])
	k := len(word)

	if n*m < k {
		return false
	}

	status := make([][]Status, n)
	for i := 0; i < n; i++ {
		status[i] = make([]Status, m)
	}

	var stack [][]int

	// stack.push([firstCandidate1, firstCandidate2, ...])
	var queue []int
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if board[i][j] == word[0] {
				queue = append(queue, toIdx(i, j, m))
			}
		}
	}

	// If the initial queue is empty, there's no possibility that the word can be found. (Actually it can also be handled inside the loop)
	if len(queue) == 0 {
		return false
	}

	stack = append(stack, queue)

	var curTrace []int

	for len(stack) != 0 {
		// queue := stack.peek()
		queue := stack[len(stack)-1]
		// Clear empty queue
		if len(queue) == 0 {
			// stack.pop()
			stack = stack[:len(stack)-1]

			if len(stack) == 1 && len(stack[0]) == 0 {
				return false
			}

			iRec, jRec := toCoord(curTrace[len(curTrace)-1], m)
			status[iRec][jRec] = Pending
			// curTrace.pop()
			curTrace = curTrace[:len(curTrace)-1]
			continue
		}

		// iCur, jCur := toCoord(queue.removeFirst(), m)
		iCur, jCur := toCoord(queue[0], m)
		stack[len(stack)-1] = queue[1:]
		if status[iCur][jCur] != Pending {
			continue
		}

		status[iCur][jCur] = Visited
		// curTrace.push(toIdx(iCur, jCur, m))
		curTrace = append(curTrace, toIdx(iCur, jCur, m))

		// If the trace can cover `word`, a match is found
		curWordIdx := len(curTrace)
		if k == curWordIdx {
			return true
		}

		// Put adjacent candidates into a queue and push the queue into the stack
		var adjCandidates []int
		if jCur-1 >= 0 && board[iCur][jCur-1] == word[curWordIdx] {
			adjCandidates = append(adjCandidates, toIdx(iCur, jCur-1, m))
		}
		if iCur-1 >= 0 && board[iCur-1][jCur] == word[curWordIdx] {
			adjCandidates = append(adjCandidates, toIdx(iCur-1, jCur, m))
		}
		if jCur+1 < m && board[iCur][jCur+1] == word[curWordIdx] {
			adjCandidates = append(adjCandidates, toIdx(iCur, jCur+1, m))
		}
		if iCur+1 < n && board[iCur+1][jCur] == word[curWordIdx] {
			adjCandidates = append(adjCandidates, toIdx(iCur+1, jCur, m))
		}

		stack = append(stack, adjCandidates)
	}

	return false
}

func toIdx(i, j, m int) int {
	return i*m + j
}

func toCoord(idx, m int) (int, int) {
	return idx / m, idx % m
}
