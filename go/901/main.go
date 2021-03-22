package main

import "fmt"

func main() {
	ss := Constructor()
	// Expecting 1
	fmt.Println(ss.Next(100))
	// Expecting 1
	fmt.Println(ss.Next(80))
	// Expecting 1
	fmt.Println(ss.Next(60))
	// Expecting 2
	fmt.Println(ss.Next(70))
	// Expecting 1
	fmt.Println(ss.Next(60))
	// Expecting 4
	fmt.Println(ss.Next(75))
	// Expecting 6
	fmt.Println(ss.Next(85))
}

type StockSpanner struct {
	prices  []int
	weights []int
}

func Constructor() StockSpanner {
	return StockSpanner{
		prices:  []int{},
		weights: []int{},
	}
}

func (s *StockSpanner) Next(price int) int {
	weight := 1
	for len(s.prices) > 0 && s.prices[len(s.prices)-1] <= price {
		s.prices = s.prices[:len(s.prices)-1]
		weight += s.weights[len(s.weights)-1]
		s.weights = s.weights[:len(s.weights)-1]
	}

	s.prices = append(s.prices, price)
	s.weights = append(s.weights, weight)

	return weight
}
