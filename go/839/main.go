package main

import "fmt"

func main() {
	strs1 := []string{"tars", "rats", "arts", "star"}
	strs2 := []string{"omv", "ovm"}

	// Expecting 2
	fmt.Println(numSimilarGroups(strs1))
	// Expecting 1
	fmt.Println(numSimilarGroups(strs2))
}

func numSimilarGroups(strs []string) int {
	n := len(strs)
	uf := newUnionFind(n)

	for i, str1 := range strs {
		for j := i + 1; j < n; j++ {
			str2 := strs[j]
			isSimilar := checkIfSimilar(str1, str2)

			if isSimilar {
				isAccepted := uf.union(i, j)
				if isAccepted && uf.areaCnt == 1 {
					return 1
				}
			}
		}
	}

	return uf.areaCnt
}

func checkIfSimilar(s, t string) bool {
	diff := 0

	for i := 0; i < len(s); i++ {
		if s[i] != t[i] {
			diff++
		}

		if diff > 2 {
			return false
		}
	}

	return true
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

	ret := UnionFind{
		parents: parents,
		ranks:   ranks,
		areaCnt: capacity,
	}

	return &ret
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
