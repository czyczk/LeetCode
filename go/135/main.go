package main

func main() {
	r1 := []int{1, 0, 2}
	r2 := []int{1, 2, 2}
	r3 := []int{1, 3, 2, 2, 1}

	// Expecting 5
	println(candy(r1))
	// Expecting 4
	println(candy(r2))
	// Expecting 7
	println(candy(r3))
}

func candy(ratings []int) int {
	n := len(ratings)
	if n == 1 {
		return 1
	}

	numCandies := make([]int, n)
	if ratings[0] != ratings[1] {
		if ratings[0] > ratings[1] {
			numCandies[0]++
		} else {
			numCandies[1]++
		}
	}

	for i := 2; i < n; i++ {
		if ratings[i] > ratings[i-1] {
			numCandies[i] = numCandies[i-1] + 1
		}
	}

	for i := n - 2; i >= 0; i-- {
		if ratings[i] > ratings[i+1] && numCandies[i] <= numCandies[i+1] {
			numCandies[i] = numCandies[i+1] + 1
		}
	}

	sum := 0
	for _, num := range numCandies {
		sum += num
	}

	return sum + n
}
