fun main(args: Array<String>) {
    val nums = intArrayOf(2, 7, 11, 5)
    val target = 9
    val result = Solution().twoSum(nums, target)
    result.forEach { println(it) }
}

class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        var resultI: Int = 0
        var resultJ: Int = 0
        for (i in 0 until nums.count() - 1) {
            for (j in i + 1 until nums.count()) {
                if (nums[i] + nums[j] == target) {
                    resultI = i
                    resultJ = j
                    break
                }
            }
        }
        return intArrayOf(resultI, resultJ)
    }
}