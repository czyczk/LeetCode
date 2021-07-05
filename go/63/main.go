package main

func main() {
	og1 := [][]int{
		{0, 0, 0},
		{0, 1, 0},
		{0, 0, 0},
	}

	og2 := [][]int{
		{0, 1},
		{0, 0},
	}

	og3 := [][]int{{1, 0}}

	og4 := [][]int{{1}}

	// Expecting 2
	println(uniquePathsWithObstacles(og1))
	// Expecting 1
	println(uniquePathsWithObstacles(og2))
	// Expecting 0
	println(uniquePathsWithObstacles(og3))
	// Expecting 0
	println(uniquePathsWithObstacles(og4))
}

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	m := len(obstacleGrid)
	n := len(obstacleGrid[0])

	f := make([][]int, m)

	for i := 0; i < m; i++ {
		f[i] = make([]int, n)
	}

	f[0][0] = 1 - obstacleGrid[0][0]

	for i := 1; i < m; i++ {
		if obstacleGrid[i][0] == 0 && f[i-1][0] != 0 {
			f[i][0] = 1
		}
	}

	for j := 1; j < n; j++ {
		if obstacleGrid[0][j] == 0 && f[0][j-1] != 0 {
			f[0][j] = 1
		}
	}

	directionsI := []int{0, -1}
	directionsJ := []int{-1, 0}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if obstacleGrid[i][j] == 1 {
				continue
			}

			for k := 0; k < 2; k++ {
				refI := i + directionsI[k]
				refJ := j + directionsJ[k]
				f[i][j] += f[refI][refJ]
			}
		}
	}

	return f[m-1][n-1]
}
