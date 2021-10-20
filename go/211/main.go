package main

import "strings"

func main() {
	wd1 := Constructor()
	wd1.AddWord("bad")
	wd1.AddWord("dad")
	wd1.AddWord("mad")
	// Expecting false
	println(wd1.Search("pad"))
	// Expecting true
	println(wd1.Search("bad"))
	// Expecting true
	println(wd1.Search(".ad"))
	// Expecting true
	println(wd1.Search("b.."))
}

type WordDictionary struct {
	trieRoot *Trie
	set      map[string]bool
}

func Constructor() WordDictionary {
	return WordDictionary{
		trieRoot: newTrie(),
		set:      make(map[string]bool),
	}
}

func (this *WordDictionary) AddWord(word string) {
	n := len(word)

	cur := this.trieRoot
	for i := 0; i < n; i++ {
		ch := word[i]
		child, ok := cur.children[ch]
		if !ok {
			child = newTrie()
			cur.children[ch] = child
		}
		cur = child

		if i == n-1 {
			cur.word = word
		}
	}

	this.set[word] = true
}

func (this *WordDictionary) Search(word string) bool {
	if !strings.ContainsRune(word, '.') {
		_, ok := this.set[word]
		return ok
	}

	return this.searchRec([]byte(word), 0, this.trieRoot)
}

func (this *WordDictionary) searchRec(pattern []byte, startIdx int, root *Trie) bool {
	if startIdx == len(pattern) {
		return root.word != ""
	}

	ch := pattern[startIdx]
	if ch != '.' {
		child, ok := root.children[ch]
		if !ok {
			return false
		}
		return this.searchRec(pattern, startIdx+1, child)
	} else {
		if len(root.children) == 0 {
			return false
		}

		for _, child := range root.children {
			if this.searchRec(pattern, startIdx+1, child) {
				return true
			}
		}

		return false
	}
}

type Trie struct {
	children map[byte]*Trie
	word     string
}

func newTrie() *Trie {
	return &Trie{
		children: make(map[byte]*Trie),
		word:     "",
	}
}
