fun main(args: Array<String>) {
    val nums = intArrayOf(2, 7, 11, 5)
    val target = 9
    val result = BestSolution().twoSum(nums, target)
    result.forEach { println(it) }
}

class BestSolution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val map = HashMap<Int, Int>()
        var idx = 0
        nums.forEach { num ->
            val complement = target - num
            val position = map[complement]
            if (position != null) {
                return intArrayOf(position, idx)
            }
            map[num] = idx++
        }
        throw Exception()
    }
}