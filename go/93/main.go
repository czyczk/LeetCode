package main

import (
	"93/solution/lp"
	"93/solution/rec"
	"fmt"
)

func main() {
	s1 := "25525511135"
	s2 := "0000"
	s3 := "1111"
	s4 := "010010"
	s5 := "101023"

	// Expecting ["255.255.11.135", "255.255.111.35"]
	fmt.Printf("%v\n", lp.RestoreIpAddresses(s1))
	fmt.Printf("%v\n", rec.RestoreIpAddresses(s1))
	// Expecting ["0.0.0.0"]
	fmt.Printf("%v\n", lp.RestoreIpAddresses(s2))
	fmt.Printf("%v\n", rec.RestoreIpAddresses(s2))
	// Expecting ["1.1.1.1"]
	fmt.Printf("%v\n", lp.RestoreIpAddresses(s3))
	fmt.Printf("%v\n", rec.RestoreIpAddresses(s3))
	// Expecting ["0.10.0.10", "0.100.1.0"]
	fmt.Printf("%v\n", lp.RestoreIpAddresses(s4))
	fmt.Printf("%v\n", rec.RestoreIpAddresses(s4))
	// Expecting ["1.0.10.23", "1.0.102.3", "10.1.0.23", "10.10.2.3", "101.0.2.3"]
	fmt.Printf("%v\n", lp.RestoreIpAddresses(s5))
	fmt.Printf("%v\n", rec.RestoreIpAddresses(s5))
}
