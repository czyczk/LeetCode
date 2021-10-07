fun main(args: Array<String>) {
    val s = Solution()

    val s1 = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
    val s2 = "AAAAAAAAAAAAA"

    // Expecting ["AAAAACCCCC","CCCCCAAAAA"]
    println(s.findRepeatedDnaSequences(s1))
    // Expecting ["AAAAAAAAAA"]
    println(s.findRepeatedDnaSequences(s2))
}

class Solution {
    fun findRepeatedDnaSequences(s: String): List<String> {
        val n = s.count()
        if (n <= 10) {
            return listOf()
        }

        val occurrenceSet = mutableSetOf<String>()
        val ans = mutableSetOf<String>()

        for (i in 0..n - 10) {
            val substr = s.substring(i, i + 10)
            if (substr in occurrenceSet) {
                ans.add(substr)
            }

            occurrenceSet.add(substr)
        }

        return ans.toList()
    }
}