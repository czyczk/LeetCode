package main

func main() {
	m1, n1 := 3, 7
	m2, n2 := 3, 2
	m3, n3 := 7, 3
	m4, n4 := 3, 3

	// Expecting 28
	println(uniquePaths(m1, n1))
	// Expecting 3
	println(uniquePaths(m2, n2))
	// Expecting 28
	println(uniquePaths(m3, n3))
	// Expecting 6
	println(uniquePaths(m4, n4))
}

func uniquePaths(m, n int) int {
	f := make([][]int, m)
	for i := 0; i < m; i++ {
		f[i] = make([]int, n)
		f[i][0] = 1
	}

	for j := 0; j < n; j++ {
		f[0][j] = 1
	}

	directionsI := []int{0, -1}
	directionsJ := []int{-1, 0}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			for k := 0; k < 2; k++ {
				refI := i + directionsI[k]
				refJ := j + directionsJ[k]
				f[i][j] += f[refI][refJ]
			}
		}
	}

	return f[m-1][n-1]
}
