package hashset

var set map[[10]int]bool

func ReorderedPowerOf2(n int) bool {
	preprocess()
	return set[getFreqCount(n)]
}

func preprocess() {
	if len(set) != 0 {
		return
	}

	set = make(map[[10]int]bool)
	// From 1 to 2^29
	num := 1
	for i := 0; i < 30; i++ {
		freqCount := getFreqCount(num)
		set[freqCount] = true
		num <<= 1
	}
}

func getFreqCount(num int) [10]int {
	temp := num
	freqCount := [10]int{}
	for temp != 0 {
		freqCount[temp%10]++
		temp /= 10
	}

	return freqCount
}
