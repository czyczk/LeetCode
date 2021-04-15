package main

import "fmt"

func main() {
	b1 := [][]byte{
		{'A', 'B', 'C', 'E'},
		{'S', 'F', 'C', 'S'},
		{'A', 'D', 'E', 'E'},
	}
	w1 := "ABCCED"

	b2 := b1
	w2 := "SEE"

	b3 := b1
	w3 := "ABCB"

	// Expcting true
	fmt.Println(exist(b1, w1))
	// Expcting true
	fmt.Println(exist(b2, w2))
	// Expecting false
	fmt.Println(exist(b3, w3))
}

type Status int

const (
	Pending Status = iota
	Visited
)

func exist(board [][]byte, word string) bool {
	m := len(board)
	n := len(board[0])
	k := len(word)

	if m*n < k {
		return false
	}

	status := make([][]Status, m)
	for i := 0; i < m; i++ {
		status[i] = make([]Status, n)
	}

	var queue []int
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if board[i][j] == word[0] {
				queue = append(queue, toIdx(i, j, n))
			}
		}
	}

	if len(queue) == 0 {
		return false
	}

	stack := [][]int{queue}
	var curTrace []int

	for len(stack) != 0 {
		queue = stack[len(stack)-1]
		if len(queue) == 0 {
			if len(curTrace) == 0 {
				return false
			}
			idxRec := curTrace[len(curTrace)-1]
			curTrace = curTrace[:len(curTrace)-1]
			iRec, jRec := toCoord(idxRec, n)
			status[iRec][jRec] = Pending

			stack = stack[:len(stack)-1]
			continue
		}

		if len(curTrace) == k-1 {
			return true
		}

		idxCur := queue[0]
		iCur, jCur := toCoord(idxCur, n)
		stack[len(stack)-1] = queue[1:]

		curTrace = append(curTrace, idxCur)
		status[iCur][jCur] = Visited

		curWordIdx := len(curTrace)
		var adjCandidates []int

		if jCur-1 >= 0 && board[iCur][jCur-1] == word[curWordIdx] && status[iCur][jCur-1] != Visited {
			adjCandidates = append(adjCandidates, toIdx(iCur, jCur-1, n))
		}
		if iCur-1 >= 0 && board[iCur-1][jCur] == word[curWordIdx] && status[iCur-1][jCur] != Visited {
			adjCandidates = append(adjCandidates, toIdx(iCur-1, jCur, n))
		}
		if jCur+1 < n && board[iCur][jCur+1] == word[curWordIdx] && status[iCur][jCur+1] != Visited {
			adjCandidates = append(adjCandidates, toIdx(iCur, jCur+1, n))
		}
		if iCur+1 < m && board[iCur+1][jCur] == word[curWordIdx] && status[iCur+1][jCur] != Visited {
			adjCandidates = append(adjCandidates, toIdx(iCur+1, jCur, n))
		}

		stack = append(stack, adjCandidates)
	}

	return false
}

func toCoord(idx int, n int) (int, int) {
	return idx / n, idx % n
}

func toIdx(i, j, n int) int {
	return i*n + j
}
