package main

func main() {
	f1 := []int{1, 2, 1}
	f2 := []int{0, 1, 2, 2}
	f3 := []int{1, 2, 3, 2, 2}
	f4 := []int{3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4}

	// Expecting 3
	println(totalFruit(f1))
	// Expecting 3
	println(totalFruit(f2))
	// Expecting 4
	println(totalFruit(f3))
	// Expecting 5
	println(totalFruit(f4))
}

func totalFruit(fruits []int) int {
	n := len(fruits)
	numTypeTaken := make([]int, n)
	var basketTypes []int

	l := 0
	maxTaken := 1
	numTypeTaken[fruits[l]]++
	basketTypes = append(basketTypes, fruits[l])

	for r := 1; r < n; r++ {
		isIn := isIn(fruits[r], basketTypes)
		for len(basketTypes) == 2 && !isIn {
			// shrink
			fruitL := fruits[l]
			numTypeTaken[fruitL]--
			if numTypeTaken[fruitL] == 0 {
				if basketTypes[0] == fruitL {
					basketTypes = basketTypes[1:]
				} else {
					basketTypes = basketTypes[0:1]
				}
			}
			l++
		}

		if !isIn {
			basketTypes = append(basketTypes, fruits[r])
		}

		numTypeTaken[fruits[r]]++
		maxTaken = max(maxTaken, r-l+1)
	}

	return maxTaken
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}

func isIn(target int, types []int) bool {
	for _, t := range types {
		if target == t {
			return true
		}
	}

	return false
}
