package hashset

func LongestConsecutive(nums []int) int {
	set := make(map[int]bool)
	for _, num := range nums {
		set[num] = true
	}

	currentStreak := 0
	longestStreak := 0

	for num := range set {
		_, ok := set[num-1]
		if ok {
			continue
		}

		currentStreak = 1

		k := num + 1
		for {
			_, ok = set[k]
			if !ok {
				break
			}

			currentStreak++
			k++
		}

		longestStreak = max(longestStreak, currentStreak)
	}

	return longestStreak
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
