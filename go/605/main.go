package main

import "fmt"

func main() {
	f1, n1 := []int{1, 0, 0, 0, 1}, 1
	f2, n2 := []int{1, 0, 0, 0, 1}, 2
	f3, n3 := []int{0, 0, 1}, 1
	f4, n4 := []int{1, 0, 0}, 1
	f5, n5 := []int{1, 0, 0, 1, 0}, 1
	f6, n6 := []int{1}, 0
	f7, n7 := []int{1, 0, 0, 0, 0}, 2
	f8, n8 := []int{0, 0}, 2
	f9, n9 := []int{0, 0, 0, 0}, 3

	// Expecting true
	fmt.Println(canPlaceFlowers(f1, n1))
	// Expecting false
	fmt.Println(canPlaceFlowers(f2, n2))
	// Expecting true
	fmt.Println(canPlaceFlowers(f3, n3))
	// Expecting true
	fmt.Println(canPlaceFlowers(f4, n4))
	// Expecting false
	fmt.Println(canPlaceFlowers(f5, n5))
	// Expecting true
	fmt.Println(canPlaceFlowers(f6, n6))
	// Expecting true
	fmt.Println(canPlaceFlowers(f7, n7))
	// Expecting false
	fmt.Println(canPlaceFlowers(f8, n8))
	// Expecting false
	fmt.Println(canPlaceFlowers(f9, n9))
}

func canPlaceFlowers(flowerbed []int, n int) bool {
	flowerbed = append(flowerbed, 0)

	l := len(flowerbed)
	if n == 0 {
		return true
	}

	if l == 0 {
		return true
	} else if l == 1 {
		return flowerbed[0] == 0 && n <= 1
	} else if l == 2 {
		return flowerbed[0]+flowerbed[1] == 0 && n <= 1
	}

	capacity := 0

	// Before the first 1, find if there's 00
	i := 2
	windowSum := flowerbed[0] + flowerbed[1]
	if windowSum == 0 {
		capacity++
		i = 3
	}

	for i < l {
		windowSum = flowerbed[i-2] + flowerbed[i-1] + flowerbed[i]

		if windowSum == 0 {
			capacity++
			i += 2
			continue
		}

		i++
	}

	return capacity >= n
}
