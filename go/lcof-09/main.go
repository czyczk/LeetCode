package main

import "fmt"

func main() {
	q1 := Constructor()
	q1.AppendTail(3)
	// Expecting 3
	fmt.Println(q1.DeleteHead())
	// Expecting -1
	fmt.Println(q1.DeleteHead())

	q2 := Constructor()
	// Expecting -1
	fmt.Println(q2.DeleteHead())
	q2.AppendTail(5)
	q2.AppendTail(2)
	// Expecting 5
	fmt.Println(q2.DeleteHead())
	// Expecting 2
	fmt.Println(q2.DeleteHead())
}

type CQueue struct {
	inStack  []int
	outStack []int
}

func Constructor() CQueue {
	return CQueue{
		inStack:  []int{},
		outStack: []int{},
	}
}

func (this *CQueue) AppendTail(value int) {
	this.inStack = append(this.inStack, value)
}

func (this *CQueue) DeleteHead() int {
	ret := -1
	lenOutStack := len(this.outStack)
	if lenOutStack == 0 {
		for len(this.inStack) != 0 {
			lenInStack := len(this.inStack)
			this.outStack = append(this.outStack, this.inStack[lenInStack-1])
			this.inStack = this.inStack[:lenInStack-1]
		}
	}

	lenOutStack = len(this.outStack)
	if lenOutStack != 0 {
		ret = this.outStack[lenOutStack-1]
		this.outStack = this.outStack[0 : lenOutStack-1]
	}

	return ret
}
