package main

func main() {
	d1 := []int{2, 1, 1, 2}
	d2 := []int{1, 2, 3, 4}

	// Expecting true
	println(isSelfCrossing(d1))
	// Expecting false
	println(isSelfCrossing(d2))
}

var directions [4][]int = [4][]int{{0, -1}, {-1, 0}, {0, 1}, {1, 0}}

func isSelfCrossing(distances []int) bool {
	visited := make(map[[2]int]bool)
	cursor := [2]int{0, 0}
	visited[cursor] = true

	for i, distance := range distances {
		direction := directions[i%4]
		for j := 1; j <= distance; j++ {
			dest := [2]int{cursor[0] + direction[0]*j, cursor[1] + direction[1]*j}
			if visited[dest] {
				return true
			}

			visited[dest] = true
		}

		cursor[0] += direction[0] * distance
		cursor[1] += direction[1] * distance
	}

	return false
}
