fun main(args: Array<String>) {
    val s = Solution()

    // Expecting 5
    println(s.countSegments("Hello, my name is John"))
    // Expecting 1
    println(s.countSegments("Hello"))
    // Expecting 4
    println(s.countSegments("love live! mu'sic forever"))
    // Expecting 0
    println(s.countSegments(""))
}

class Solution {
    fun countSegments(s: String): Int {
        var count = 0
        var lastCh = ' '
        for (ch in s) {
            if (ch != ' ' && lastCh == ' ') {
                count++
            }

            lastCh = ch
        }

        return count
    }
}