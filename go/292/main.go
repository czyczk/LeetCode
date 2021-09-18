package main

func main() {
	// Expecting false
	println(canWinNim(4))
	// Expecting true
	println(canWinNim(1))
	// Expecting true
	println(canWinNim(2))
}

func canWinNim(n int) bool {
	return n%4 != 0
}
