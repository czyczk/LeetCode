package main

import "fmt"

func main() {
	n1 := 4
	edges1 := [][]int{{3, 1, 2}, {3, 2, 3}, {1, 1, 3}, {1, 2, 4}, {1, 1, 2}, {2, 3, 4}}
	n2 := 4
	edges2 := [][]int{{3, 1, 2}, {3, 2, 3}, {1, 1, 4}, {2, 1, 4}}
	n3 := 4
	edges3 := [][]int{{3, 2, 3}, {1, 1, 2}, {2, 3, 4}}

	// Expecting 2
	fmt.Println(maxNumEdgesToRemove(n1, edges1))
	// Expecting 0
	fmt.Println(maxNumEdgesToRemove(n2, edges2))
	// Expecting -1
	fmt.Println(maxNumEdgesToRemove(n3, edges3))
}

func maxNumEdgesToRemove(n int, edges [][]int) int {
	if n == 0 {
		return 0
	}

	numRemoved := 0

	// Union find Alice is the common uf for now
	// Add the shared edges to the uf
	ufA := newUnionFind(n)
	for _, edge := range edges {
		if edge[0] == 3 {
			isAccepted := ufA.union(edge[1]-1, edge[2]-1)
			if !isAccepted {
				numRemoved++
			}
		}
	}

	// Now make a clone of Alice' uf and consider their own edges
	ufB := ufA.clone()

	for _, edge := range edges {
		var isAccepted bool

		if edge[0] == 1 {
			isAccepted = ufA.union(edge[1]-1, edge[2]-1)
		} else if edge[0] == 2 {
			isAccepted = ufB.union(edge[1]-1, edge[2]-1)
		} else {
			continue
		}

		if !isAccepted {
			numRemoved++
		}
	}

	if ufA.areaCnt != 1 || ufB.areaCnt != 1 {
		return -1
	}
	return numRemoved
}

type UnionFind struct {
	parents []int
	ranks   []int
	areaCnt int
}

func newUnionFind(capacity int) *UnionFind {
	parents := make([]int, capacity)
	ranks := make([]int, capacity)
	for i := 0; i < capacity; i++ {
		parents[i] = i
		ranks[i] = 1
	}

	return &UnionFind{
		parents: parents,
		ranks:   ranks,
		areaCnt: capacity,
	}
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

	uf.areaCnt--
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

func (uf *UnionFind) clone() *UnionFind {
	parentsCloned := make([]int, len(uf.parents))
	ranksCloned := make([]int, len(uf.ranks))
	copy(parentsCloned, uf.parents)
	copy(ranksCloned, uf.ranks)

	return &UnionFind{
		parents: parentsCloned,
		ranks:   ranksCloned,
		areaCnt: uf.areaCnt,
	}
}
