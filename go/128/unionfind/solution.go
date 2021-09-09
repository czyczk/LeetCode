package unionfind

func LongestConsecutive(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	elIdxMap := make(map[int]int)
	ufCapacity := 0

	for _, num := range nums {
		_, ok := elIdxMap[num]
		if !ok {
			elIdxMap[num] = ufCapacity
			ufCapacity++
		}

		_, ok = elIdxMap[num-1]
		if !ok {
			elIdxMap[num-1] = ufCapacity
			ufCapacity++
		}

	}

	uf := NewUnionFind(ufCapacity)
	for _, num := range nums {
		uf.Union(elIdxMap[num], elIdxMap[num-1])
	}

	maxSize := 0
	for i := 0; i < ufCapacity; i++ {
		size := uf.GetSize(i)
		if size > maxSize {
			maxSize = size
		}
	}

	return maxSize - 1
}

type UnionFind struct {
	capacity int
	parents  []int
	sizes    []int
}

func NewUnionFind(capacity int) *UnionFind {
	parents := make([]int, capacity)
	sizes := make([]int, capacity)
	for i := 0; i < capacity; i++ {
		parents[i] = i
		sizes[i] = 1
	}

	return &UnionFind{
		capacity: capacity,
		parents:  parents,
		sizes:    sizes,
	}
}

func (uf *UnionFind) GetCapacity() int {
	return uf.capacity
}

func (uf *UnionFind) IsConnected(x, y int) bool {
	return uf.Find(x) == uf.Find(y)
}

func (uf *UnionFind) Union(x, y int) bool {
	if x >= uf.capacity || y >= uf.capacity {
		return false
	}

	parentX := uf.Find(x)
	parentY := uf.Find(y)
	if parentX == parentY {
		return false
	}

	uf.parents[x] = parentY
	uf.sizes[parentY] += uf.sizes[x]

	return true
}

func (uf *UnionFind) Find(x int) int {
	if x >= uf.capacity {
		panic("out of capacity")
	}

	oldParent := uf.parents[x]
	if x == oldParent {
		return x
	}

	uf.parents[x] = uf.Find(oldParent)
	return uf.parents[x]
}

func (uf *UnionFind) GetSize(x int) int {
	if x >= uf.capacity {
		panic("out of capacity")
	}

	return uf.sizes[uf.Find(x)]
}
