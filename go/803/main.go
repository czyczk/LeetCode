package main

import "fmt"

func main() {
	grid1 := [][]int{{1, 0, 0, 0}, {1, 1, 1, 0}}
	hits1 := [][]int{{1, 0}}
	grid2 := [][]int{{1, 0, 0, 0}, {1, 1, 0, 0}}
	hits2 := [][]int{{1, 1}, {1, 0}}
	grid3 := [][]int{{1}, {1}, {1}, {1}, {1}}
	hits3 := [][]int{{3, 0}, {4, 0}, {1, 0}, {2, 0}, {0, 0}}

	// Expecting [2]
	fmt.Println(hitBricks(grid1, hits1))
	// Expecting [0, 0]
	fmt.Println(hitBricks(grid2, hits2))
	// Expecting [1, 0, 1, 0, 0]
	fmt.Println(hitBricks(grid3, hits3))
}

func hitBricks(grid [][]int, hits [][]int) []int {
	m := len(grid)
	n := len(grid[0])
	size := m * n

	// Transcribe the grid to a brick book
	bb := newBrickBook(m, n)
	for x, row := range grid {
		for y, existence := range row {
			if existence == 1 {
				idx := bb.toIdx(x, y)
				bb.turnOn(idx)
			}
		}
	}

	// Turn off all the hit points from the brick book first
	for _, hit := range hits {
		idx := bb.toIdx(hit[0], hit[1])
		bb.turnOff(idx)
	}

	uf := newUnionFind(size)
	// Union all the top bricks to the "top"
	for y := 0; y < n; y++ {
		idxCur := bb.toIdx(0, y)
		if bb.arr[idxCur] == 1 {
			uf.union(idxCur, size)
		}
	}

	// Build a union find from the brick book
	for x := 0; x < m; x++ {
		for y := 0; y < n; y++ {
			idxCur := bb.toIdx(x, y)
			if bb.arr[idxCur] == 0 {
				continue
			}

			// Union adjacent bricks
			unionUp(x, y, bb, uf)
			unionLeft(x, y, bb, uf)
		}
	}

	nHits := len(hits)
	ret := make([]int, nHits)
	// Recover the hit points in reverse order
	for i := nHits - 1; i >= 0; i-- {
		x := hits[i][0]
		y := hits[i][1]
		if grid[x][y] == 0 {
			continue
		}

		idxCur := bb.toIdx(x, y)
		bb.turnOn(idxCur)

		numOriCount := uf.counts[uf.find(size)]

		// Union to the "top" if it's a top brick
		if x == 0 {
			uf.union(idxCur, size)
		}

		// Union adjacent bricks
		unionUp(x, y, bb, uf)
		unionLeft(x, y, bb, uf)
		unionDown(x, y, bb, uf)
		unionRight(x, y, bb, uf)

		ret[i] = max(0, uf.counts[uf.find(size)]-numOriCount-1)
	}

	return ret
}

func max(a, b int) int {
	if a < b {
		return b
	}

	return a
}

// Union current and up if there's a brick
func unionUp(x, y int, bb *BrickBook, uf *UnionFind) {
	if x <= 0 {
		return
	}

	idxUp := bb.toIdx(x-1, y)
	if bb.arr[idxUp] == 0 {
		return
	}

	idxCur := bb.toIdx(x, y)
	uf.union(idxCur, idxUp)
}

// Union current and left if there's a brick
func unionLeft(x, y int, bb *BrickBook, uf *UnionFind) {
	if y <= 0 {
		return
	}

	idxLeft := bb.toIdx(x, y-1)
	if bb.arr[idxLeft] == 0 {
		return
	}

	idxCur := bb.toIdx(x, y)
	uf.union(idxCur, idxLeft)
}

// Union current and down if there's a brick
func unionDown(x, y int, bb *BrickBook, uf *UnionFind) {
	if x+1 >= bb.m {
		return
	}

	idxDown := bb.toIdx(x+1, y)
	if bb.arr[idxDown] == 0 {
		return
	}

	idxCur := bb.toIdx(x, y)
	uf.union(idxCur, idxDown)
}

// Union current and right if there's a brick
func unionRight(x, y int, bb *BrickBook, uf *UnionFind) {
	if y+1 >= bb.n {
		return
	}

	idxRight := bb.toIdx(x, y+1)
	if bb.arr[idxRight] == 0 {
		return
	}

	idxCur := bb.toIdx(x, y)
	uf.union(idxCur, idxRight)
}

// BrickBook is a brick book
type BrickBook struct {
	arr []int
	m   int
	n   int
}

func newBrickBook(m, n int) *BrickBook {
	ret := BrickBook{
		arr: make([]int, m*n),
		m:   m,
		n:   n,
	}

	return &ret
}

func (b *BrickBook) toIdx(x, y int) int {
	return x*b.n + y
}

func (b *BrickBook) toPoint(idx int) (x, y int) {
	x = idx / b.n
	y = idx % b.n
	return x, y
}

func (b *BrickBook) turnOn(idx int) {
	b.arr[idx] = 1
}

func (b *BrickBook) turnOff(idx int) {
	b.arr[idx] = 0
}

// UnionFind is a union find
type UnionFind struct {
	parents []int
	counts  []int
}

func newUnionFind(size int) *UnionFind {
	ret := &UnionFind{
		parents: make([]int, size+1),
		counts:  make([]int, size+1),
	}

	for i := 0; i <= size; i++ {
		ret.parents[i] = i
		ret.counts[i] = 1
	}

	return ret
}

func (uf *UnionFind) union(x int, y int) {
	rootX := uf.find(x)
	rootY := uf.find(y)

	if rootX == rootY {
		return
	}

	uf.parents[rootX] = rootY

	countRootX := uf.counts[rootX]
	countRootY := uf.counts[rootY]
	uf.counts[rootY] = countRootX + countRootY

	return
}

func (uf *UnionFind) find(x int) int {
	oriParent := uf.parents[x]
	if oriParent == x {
		return x
	}

	newParent := uf.find(oriParent)
	uf.parents[x] = newParent

	return newParent
}
