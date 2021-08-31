fun main() {
	val s = Solution()

	val n1 = arrayOf(1, 2, 3, 4).toIntArray()
	val n2 = arrayOf(1).toIntArray()
	val n3 = arrayOf(1, 2, 3, 8, 9, 10).toIntArray()

	// Expecting 3
	println(s.numberOfArithmeticSlices(n1))
	// Expecting 0
	println(s.numberOfArithmeticSlices(n2))
	// Expecting 2
	println(s.numberOfArithmeticSlices(n3))
}

class Solution {
	fun numberOfArithmeticSlices(nums: IntArray): Int {
		val n = nums.count();
		if (n < 3) {
			return 0
		}

		var ret = 0

		var diff = nums[1] - nums[0]
		var start = 0
		var end = 1

		while (end < n) {
			val newDiff = nums[end] - nums[end - 1]
			if (newDiff != diff) {
				ret += getCount(end - start)
				start = end - 1
				diff = newDiff
			}

			end++
		}

		if (nums[n - 1] - nums[n - 2] == diff) {
			ret += getCount(n - start)
		}

		return ret
	}

	fun getCount(n: Int): Int {
		if (n < 3) {
			return 0
		}

		return (n - 2) * (n - 1) / 2
	}
}
