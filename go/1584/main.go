package main

import (
	"fmt"
	"sort"
)

func main() {
	points1 := [][]int{{0, 0}, {2, 2}, {3, 10}, {5, 2}, {7, 0}}
	points2 := [][]int{{3, 12}, {-2, 5}, {-4, 1}}
	points3 := [][]int{{-1000000, -1000000}, {1000000, 1000000}}
	points4 := [][]int{{0, 0}}

	// Expecting 20
	fmt.Println(minCostConnectPoints(points1))
	// Expecting 18
	fmt.Println(minCostConnectPoints(points2))
	// Expecting 4000000
	fmt.Println(minCostConnectPoints(points3))
	// Expecting 0
	fmt.Println(minCostConnectPoints(points4))
}

func minCostConnectPoints(points [][]int) int {
	n := len(points)
	if n == 1 {
		return 0
	}

	edges := []Edge{}
	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			edges = append(edges, newEdge(points[i], points[j]))
		}
	}

	sort.Slice(edges, func(i, j int) bool { return edges[i].len < edges[j].len })

	uf := newUnionFind()
	for i := 0; i < n; i++ {
		uf.find(fmt.Sprintf("%v", points[i]))
	}

	numPoints := len(uf.parents)
	numAccepcted := 0
	ret := 0
	for _, edge := range edges {
		isAccepted := uf.union(fmt.Sprintf("%v", edge.x), fmt.Sprintf("%v", edge.y))
		if isAccepted {
			ret += edge.len
			numAccepcted++
		}

		if numAccepcted == numPoints-1 {
			break
		}
	}

	return ret
}

func abs(num int) int {
	if num < 0 {
		return -num
	}
	return num
}

// Edge is an edge
type Edge struct {
	x   []int
	y   []int
	len int
}

func newEdge(p1, p2 []int) Edge {
	len := abs(p2[0]-p1[0]) + abs(p2[1]-p1[1])
	return Edge{
		x:   p1,
		y:   p2,
		len: len,
	}
}

// UnionFind is a union find
type UnionFind struct {
	parents map[string]string
}

func newUnionFind() UnionFind {
	return UnionFind{
		parents: make(map[string]string),
	}
}

func (uf *UnionFind) union(x, y string) bool {
	rootX := uf.find(x)
	rootY := uf.find(y)

	if rootX == rootY {
		return false
	}

	uf.parents[rootX] = rootY
	return true
}

func (uf *UnionFind) find(x string) string {
	oriParent, ok := uf.parents[x]
	if !ok {
		uf.parents[x] = x
		oriParent = x
	}

	if oriParent == x {
		return x
	}

	newParent := uf.find(oriParent)
	uf.parents[x] = newParent
	return newParent
}

func (uf *UnionFind) isConnected(x, y string) bool {
	return uf.find(x) == uf.find(y)
}
