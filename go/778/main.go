package main

import (
	"fmt"
	"sort"
)

func main() {
	grid1 := [][]int{{0, 2}, {1, 3}}
	grid2 := [][]int{{0, 1, 2, 3, 4}, {24, 23, 22, 21, 5}, {12, 13, 14, 15, 16}, {11, 17, 18, 19, 20}, {10, 9, 8, 7, 6}}

	// Expecting 3
	fmt.Println(swimInWater(grid1))
	// Expecting 16
	fmt.Println(swimInWater(grid2))
}

func swimInWater(grid [][]int) int {
	n := len(grid)
	if n == 0 || n == 1 {
		return 0
	}

	edges := []edge{}
	maxDepth := 0
	for i, line := range grid {
		for j, depth := range line {
			idx := toIdx(i, j, n)

			// Link to the right as an edge
			if j < n-1 {
				rightIdx := toIdx(i, j+1, n)
				largerDepth := max(depth, grid[i][j+1])
				edge := edge{
					x:     idx,
					y:     rightIdx,
					depth: largerDepth,
				}
				edges = append(edges, edge)

				maxDepth = max(maxDepth, largerDepth)
			}

			// Link to the bottom as an edge
			if i < n-1 {
				bottomIdx := toIdx(i+1, j, n)
				largerDepth := max(depth, grid[i+1][j])
				edge := edge{
					x:     idx,
					y:     bottomIdx,
					depth: largerDepth,
				}
				edges = append(edges, edge)

				maxDepth = max(maxDepth, largerDepth)
			}
		}
	}

	sort.Slice(edges, func(i, j int) bool {
		return edges[i].depth < edges[j].depth
	})

	// Use a union find to find the minimum depth to make both ends connected
	uf := newUnionFind(n * n)
	for _, edge := range edges {
		isAccepted := uf.union(edge.x, edge.y)
		if !isAccepted {
			continue
		}

		isConnected := uf.isConnected(0, n*n-1)
		if isConnected {
			return edge.depth
		}
	}

	return -1
}

func toIdx(i, j, n int) int {
	return i*n + j
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}

type edge struct {
	x, y, depth int
}

type UnionFind struct {
	parents []int
	ranks   []int
}

func newUnionFind(capacity int) *UnionFind {
	parents := make([]int, capacity)
	ranks := make([]int, capacity)

	for i := 0; i < capacity; i++ {
		parents[i] = i
		ranks[i] = 1
	}

	ret := UnionFind{
		parents: parents,
		ranks:   ranks,
	}

	return &ret
}

func (uf *UnionFind) union(x, y int) bool {
	rootX := uf.find(x)
	rootY := uf.find(y)
	if rootX == rootY {
		return false
	}

	rankX := uf.ranks[rootX]
	rankY := uf.ranks[rootY]

	if rankX < rankY {
		rootX, rootY = rootY, rootX
		rankX, rankY = rankY, rankX
	}

	uf.parents[rootY] = rootX
	uf.ranks[rootX] += rankY

	return true
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

func (uf *UnionFind) isConnected(x, y int) bool {
	return uf.find(x) == uf.find(y)
}
