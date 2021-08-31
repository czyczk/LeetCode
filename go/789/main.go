package main

func main() {
	g1 := [][]int{{1, 0}, {0, 3}}
	t1 := []int{0, 1}

	g2 := [][]int{{1, 0}}
	t2 := []int{2, 0}

	g3 := [][]int{{2, 0}}
	t3 := []int{1, 0}

	g4 := [][]int{{5, 0}, {-10, -2}, {0, -5}, {-2, -2}, {-7, 1}}
	t4 := []int{7, 7}

	g5 := [][]int{{-1, 0}, {0, 1}, {-1, 0}, {0, 1}, {-1, 0}}
	t5 := []int{0, 0}

	// Expecting true
	println(escapeGhosts(g1, t1))
	// Expecting false
	println(escapeGhosts(g2, t2))
	// Expecting false
	println(escapeGhosts(g3, t3))
	// Expecting false
	println(escapeGhosts(g4, t4))
	// Expecting true
	println(escapeGhosts(g5, t5))
}

func escapeGhosts(ghosts [][]int, target []int) bool {
	targetSteps := abs(target[0]) + abs(target[1])

	// If the ghosts is at [0, 0], return false.
	// This case is included in the following judgements.
	for _, ghost := range ghosts {
		// Ghost at the original point
		if ghost[0] == 0 && ghost[1] == 0 {
			return false
		}

		// Ghost in the way
		if ghost[0] == 0 && target[0] == 0 && abs(ghost[1]) <= abs(target[1]) {
			return false
		}

		if ghost[1] == 0 && target[1] == 0 && abs(ghost[0]) <= abs(target[0]) {
			return false
		}

		ghostStep := abs(target[0]-ghost[0]) + abs(target[1]-ghost[1])

		// The case where the ghost is at the target is included
		if ghostStep <= targetSteps {
			return false
		}
	}

	return true
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}
