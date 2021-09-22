package main

import (
	"strings"
)

func main() {
	words1 := []string{"This", "is", "an", "example", "of", "text", "justification."}
	mw1 := 16

	words2 := []string{"What", "must", "be", "acknowledgement", "shall", "be"}
	mw2 := 16

	words3 := []string{"Science", "is", "what", "we", "understand", "well", "enough", "to", "explain", "to", "a", "computer.", "Art", "is", "everything", "else", "we", "do"}
	mw3 := 20

	printResult(fullJustify(words1, mw1))
	println()
	printResult(fullJustify(words2, mw2))
	println()
	printResult(fullJustify(words3, mw3))
}

func printResult(result []string) {
	for _, line := range result {
		println(line)
	}
}

func fullJustify(words []string, maxWidth int) []string {
	var lines []Line

	var line Line
	for _, word := range words {
		if line.CanAdd(word, maxWidth) {
			line.Add(word)
		} else {
			line.isFull = true
			lines = append(lines, line)
			line = Line{}
			line.Add(word)
		}
	}

	if len(line.words) > 0 {
		lines = append(lines, line)
	}

	p := Passage{
		lines:    lines,
		maxWidth: maxWidth,
	}

	return p.ToStringSlice()
}

type Passage struct {
	lines    []Line
	maxWidth int
}

type Line struct {
	words        []string
	charCount    int
	displayCount int
	isFull       bool
}

func (l *Line) Add(word string) {
	l.charCount += len(word)
	l.displayCount += len(word)
	if len(l.words) > 0 {
		l.displayCount++
	}

	l.words = append(l.words, word)
}

func (l *Line) CanAdd(word string, maxWidth int) bool {
	if len(l.words) == 0 {
		return true
	}

	return (maxWidth - l.displayCount) >= len(word)+1
}

func (p *Passage) ToStringSlice() []string {
	var ret []string

	for _, line := range p.lines {
		if !line.isFull || len(line.words) == 1 {
			var rightPadding []byte
			for i := 0; i < p.maxWidth-line.displayCount; i++ {
				rightPadding = append(rightPadding, ' ')
			}

			ret = append(ret, strings.Join(line.words, " ")+string(rightPadding))
		} else {
			spaceLeft := p.maxWidth - line.charCount
			avg := spaceLeft / (len(line.words) - 1)
			rem := spaceLeft % (len(line.words) - 1)

			var lineStr string
			for i, word := range line.words {
				lineStr += word
				if i == len(line.words)-1 {
					continue
				}

				for j := 0; j < avg; j++ {
					lineStr += " "
				}
				if rem > 0 {
					lineStr += " "
					rem--
				}
			}

			ret = append(ret, lineStr)
		}
	}

	return ret
}
