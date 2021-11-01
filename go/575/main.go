package main

func main() {
	t1 := []int{1, 1, 2, 2, 3, 3}
	t2 := []int{1, 1, 2, 3}
	t3 := []int{6, 6, 6, 6}

	// Expecting 3
	println(distributeCandies(t1))
	// Expecting 2
	println(distributeCandies(t2))
	// Expecting 1
	println(distributeCandies(t3))
}

func distributeCandies(candyType []int) int {
	n := len(candyType)
	set := make(map[int]bool)
	for _, t := range candyType {
		set[t] = true
		if len(set) > n/2 {
			return n / 2
		}
	}

	return min(n/2, len(set))
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
