package main

import (
	"fmt"
)

func main() {
	stones1 := [][]int{
		[]int{0, 0},
		[]int{0, 1},
		[]int{1, 0},
		[]int{1, 2},
		[]int{2, 1},
		[]int{2, 2},
	}

	// Expecting 5
	fmt.Println(removeStones(stones1))
}

func removeStones(stones [][]int) int {
	n := len(stones)
	if n == 1 {
		return 0
	}

	uf := newUnionFind()
	for _, point := range stones {
		uf.union(point[0]+10001, point[1])
	}

	return n - uf.numAreas
}

type UnionFind struct {
	parents  map[int]int
	numAreas int
}

func newUnionFind() UnionFind {
	uf := UnionFind{
		parents:  make(map[int]int),
		numAreas: 0,
	}

	return uf
}

func (uf *UnionFind) union(x int, y int) {
	rootX := uf.find(x)
	rootY := uf.find(y)

	if rootX == rootY {
		return
	}

	uf.parents[rootX] = rootY
	uf.numAreas--
}

func (uf *UnionFind) find(x int) int {
	oriParent, ok := uf.parents[x]
	if !ok {
		uf.parents[x] = x
		uf.numAreas++
		oriParent = x
	}

	if oriParent == x {
		return x
	}

	newParent := uf.find(oriParent)
	uf.parents[x] = newParent

	return newParent
}
