import java.util.*

fun main(args: Array<String>) {
    val sr1 = SummaryRanges()
    sr1.addNum(1)
    // Expecting [[1, 1]]
    printResult(sr1.getIntervals())
    sr1.addNum(3)
    // Expecting [[1, 1], [3, 3]]
    printResult(sr1.getIntervals())
    sr1.addNum(7)
    // Expecting [[1, 1], [3, 3], [7, 7]]
    printResult(sr1.getIntervals())
    sr1.addNum(2)
    // Expecting [[1, 3], [7, 7]]
    printResult(sr1.getIntervals())
    sr1.addNum(6)
    // Expecting [[1, 3], [6, 7]]
    printResult(sr1.getIntervals())
}

fun printResult(result: Array<IntArray>) {
    println(result.map { it.toList() })
}

class SummaryRanges() {
    private val arr = TreeSet<Int>()

    fun addNum(`val`: Int) {
        arr.add(`val`)
    }

    fun getIntervals(): Array<IntArray> {
        val ret = mutableListOf<IntArray>()
        if (arr.isEmpty()) {
            return ret.toTypedArray()
        }

        var start: Int? = null
        var lastNum = 0

        for (num in arr) {
            if (start == null) {
                start = num
                lastNum = num
                continue
            }

            if (num != lastNum + 1) {
                ret.add(intArrayOf(start, lastNum))
                start = num
            }

            lastNum = num
        }

        ret.add(intArrayOf(start!!, lastNum))

        return ret.toTypedArray()
    }

}