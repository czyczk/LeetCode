package main

func main() {
	it1 := newIterator([]int{1, 2, 3})
	pi1 := Constructor(it1)

	// Expecting 1
	println(pi1.next())
	// Expecting 2
	println(pi1.peek())
	// Expecting 2
	println(pi1.next())
	// Expecting 3
	println(pi1.next())
	// Expecting false
	println(pi1.hasNext())
}

/*   Below is the interface for Iterator, which is already defined for you.
 *
 */
type Iterator struct {
	arr []int
	idx int
}

func newIterator(nums []int) *Iterator {
	return &Iterator{
		arr: nums,
		idx: 0,
	}
}

func (this *Iterator) hasNext() bool {
	return this.idx < len(this.arr)
}

func (this *Iterator) next() int {
	if !this.hasNext() {
		return 0
	}

	ret := this.arr[this.idx]
	this.idx++
	return ret
}

type PeekingIterator struct {
	it       *Iterator
	pHasNext bool
	nextEl   int
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{
		it:       iter,
		pHasNext: iter.hasNext(),
		nextEl:   iter.next(),
	}
}

func (this *PeekingIterator) hasNext() bool {
	return this.pHasNext
}

func (this *PeekingIterator) next() int {
	ret := this.nextEl
	this.pHasNext = this.it.hasNext()
	this.nextEl = this.it.next()

	return ret
}

func (this *PeekingIterator) peek() int {
	return this.nextEl
}
