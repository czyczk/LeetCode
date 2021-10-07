fun main() {
    val s = Solution()

    val s1 = "catsanddog"
    val wd1 = listOf("cat", "cats", "and", "sand", "dog")

    val s2 = "pineapplepenapple"
    val wd2 = listOf("apple", "pen","applepen", "pine", "pineapple")

    val s3 = "catsandog"
    val wd3 = listOf("cats", "dog", "sand", "and", "cat")

    // Expecting ["cats and dog", "cat sand dog"]
    println(s.wordBreak(s1, wd1))
    // Expecting ["pine apple pen apple", "pineapple pen apple", "pine applepen apple"]
    println(s.wordBreak(s2, wd2))
    // Expecting []
    println(s.wordBreak(s3, wd3))
}

class Solution {
    private lateinit var idxBreaksMap: MutableMap<Int, List<ArrayDeque<String>>>
    private lateinit var wordSet: Set<String>

    fun wordBreak(s: String, wordDict: List<String>): List<String> {
        idxBreaksMap = mutableMapOf()
        val wordSet = mutableSetOf<String>()
        wordDict.forEach { wordSet.add(it) }
        this.wordSet = wordSet

        val wordBreaks = backtraceRec(s, 0)
        return wordBreaks.map {
            it.joinToString(" ")
        }.toList()
    }

    private fun backtraceRec(s: String, start: Int): List<ArrayDeque<String>> {
        var ret = idxBreaksMap[start]?.toMutableList()
        if (ret != null) {
            return ret
        }

        ret = mutableListOf()
        if (start == s.count()) {
            ret.add(ArrayDeque())
        }

        for (i in start + 1..s.count()) {
            val substr = s.substring(start, i)
            if (substr !in wordSet) {
                continue
            }

            val wordBreaks = backtraceRec(s, i)
            for (it in wordBreaks) {
                val wordBreak = ArrayDeque(it)
                wordBreak.addFirst(substr)
                ret.add(wordBreak)
            }
        }

        idxBreaksMap[start] = ret
        return ret
    }
}