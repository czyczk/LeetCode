package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	n1 := 5
	edges1 := [][]int{{0, 1, 1}, {1, 2, 1}, {2, 3, 2}, {0, 3, 2}, {0, 4, 3}, {3, 4, 3}, {1, 4, 6}}
	n2 := 4
	edges2 := [][]int{{0, 1, 1}, {1, 2, 1}, {2, 3, 1}, {0, 3, 1}}

	// Expecting [[0, 1], [2, 3, 4, 5]]
	fmt.Println(findCriticalAndPseudoCriticalEdges(n1, edges1))
	// Expecting [[], [0, 1, 2, 3]]
	fmt.Println(findCriticalAndPseudoCriticalEdges(n2, edges2))
}

func findCriticalAndPseudoCriticalEdges(n int, edges [][]int) [][]int {
	for i, edge := range edges {
		edges[i] = append(edge, i)
	}
	sort.Slice(edges, func(i, j int) bool { return edges[i][2] < edges[j][2] })

	uf := newUnionFind(n)
	minMSTValue := calcMST(&uf, -1, edges, n)

	var keyEdges, pseudoKeyEdges []int
	for i, edge := range edges {
		uf = newUnionFind(n)
		if calcMST(&uf, i, edges, n) > minMSTValue {
			keyEdges = append(keyEdges, edge[3])
			continue
		}

		uf = newUnionFind(n)
		uf.union(edge[0], edge[1])
		mstValue := calcMST(&uf, i, edges, n-1) + edge[2]
		if mstValue == minMSTValue {
			pseudoKeyEdges = append(pseudoKeyEdges, edge[3])
		}
	}

	return [][]int{keyEdges, pseudoKeyEdges}
}

func calcMST(uf *UnionFind, ignoreID int, edges [][]int, expectedNumEdges int) int {
	mstValue := 0
	numAccepted := 0

	for i, edge := range edges {
		if i == ignoreID {
			continue
		}

		if uf.union(edge[0], edge[1]) {
			mstValue += edge[2]
			numAccepted++
		}

		if numAccepted == expectedNumEdges-1 {
			break
		}
	}

	if uf.count > 1 {
		return math.MaxInt32
	}

	return mstValue
}

// UnionFind is a union find
type UnionFind struct {
	parents []int
	ranks   []int
	count   int
}

func newUnionFind(capacity int) UnionFind {
	ret := UnionFind{
		parents: make([]int, capacity),
		ranks:   make([]int, capacity),
		count:   capacity,
	}

	for i := 0; i < capacity; i++ {
		ret.parents[i] = i
		ret.ranks[i] = 1
	}

	return ret
}

func (uf *UnionFind) union(x, y int) bool {
	rootX := uf.find(x)
	rootY := uf.find(y)
	if rootX == rootY {
		return false
	}

	if uf.ranks[rootX] < uf.ranks[rootY] {
		rootX, rootY = rootY, rootX
	}

	uf.ranks[rootX] += uf.ranks[rootY]
	uf.parents[rootY] = rootX
	uf.count--

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
