package main

import "sort"

func main() {
	acc1 := [][]string{{"John", "johnsmith@mail.com", "john00@mail.com"},
		{"John", "johnnybravo@mail.com"},
		{"John", "johnsmith@mail.com", "john_newyork@mail.com"},
		{"Mary", "mary@mail.com"}}

	// Expecting [
	//   ["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"],
	//   ["John", "johnnybravo@mail.com"],
	//   ["Mary", "mary@mail.com"]
	// ]
	printList(accountsMerge(acc1))
}

func printList(list [][]string) {
	for _, emails := range list {
		print("[")
		isFirst := true
		for _, email := range emails {
			if !isFirst {
				print(", ")
			}
			print(email)
			isFirst = false
		}
		println("]")
	}
}

func accountsMerge(accounts [][]string) [][]string {
	emailIdxMap := make(map[string]int)
	uf := newUnionFind()
	ret := make([][]string, 0, len(accounts))

	for i, emails := range accounts {
		n := len(emails)

		uf.find(emails[1])
		emailIdxMap[emails[1]] = i

		for j := 2; j < n; j++ {
			email := emails[j]
			emailIdxMap[email] = i
			uf.union(email, emails[j-1])
		}
	}

	idxMapping := make([]int, len(accounts))
	for i := 0; i < len(accounts); i++ {
		idxMapping[i] = -1
	}

	for email := range uf.parents {
		root := uf.find(email)
		idx := idxMapping[emailIdxMap[root]]
		if idx == -1 {
			idxMapping[emailIdxMap[root]] = len(ret)
			name := accounts[emailIdxMap[root]][0]
			ret = append(ret, []string{name, email})
		} else {
			ret[idx] = append(ret[idx], email)
		}
	}

	for i := 0; i < len(ret); i++ {
		sort.Strings(ret[i][1:])
	}
	return ret
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

func (uf *UnionFind) union(x, y string) {
	rootX := uf.find(x)
	rootY := uf.find(y)
	if rootX == rootY {
		return
	}

	uf.parents[rootX] = rootY
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
