package main

import (
	"fmt"
	"sort"
)

func main() {
	heights1 := [][]int{{1, 2, 2}, {3, 8, 2}, {5, 3, 5}}
	heights2 := [][]int{{1, 2, 3}, {3, 8, 4}, {5, 3, 5}}
	heights3 := [][]int{{1, 2, 1, 1, 1}, {1, 2, 1, 2, 1}, {1, 2, 1, 2, 1}, {1, 2, 1, 2, 1}, {1, 1, 1, 2, 1}}

	// Expecting 2
	fmt.Println(minimumEffortPath(heights1))
	// Expecting 1
	fmt.Println(minimumEffortPath(heights2))
	// Expecting 0
	fmt.Println(minimumEffortPath(heights3))
}

func minimumEffortPath(heights [][]int) int {
	n := len(heights)
	m := len(heights[0])

	edges := []edge{}
	for i, line := range heights {
		for j, height := range line {
			idx := toIdx(i, j, m)
			if j < m-1 {
				e := edge{
					x:      idx,
					y:      toIdx(i, j+1, m),
					weight: abs(height - heights[i][j+1]),
				}
				edges = append(edges, e)
			}

			if i < n-1 {
				e := edge{
					x:      idx,
					y:      toIdx(i+1, j, m),
					weight: abs(height - heights[i+1][j]),
				}
				edges = append(edges, e)
			}
		}
	}

	sort.Slice(edges, func(i, j int) bool {
		return edges[i].weight < edges[j].weight
	})

	uf := newUnionFind(m * n)
	for _, edge := range edges {
		uf.union(edge.x, edge.y)
		isConnected := uf.isConnected(0, m*n-1)
		if isConnected {
			return edge.weight
		}
	}

	return 0
}

type edge struct {
	x, y   int
	weight int
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func toIdx(x, y int, m int) int {
	return x*m + y
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
	rootX := uf.find(x)
	rootY := uf.find(y)

	return rootX == rootY
}
