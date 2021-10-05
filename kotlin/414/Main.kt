fun main() {
	val s = Solution()

	val nums1 = arrayOf(3, 2, 1).toIntArray()
	val nums2 = arrayOf(1, 2).toIntArray()
	val nums3 = arrayOf(2, 2, 3, 1).toIntArray()

	// Expecting 1
	println(s.thirdMax(nums1))
	// Expecting 2
	println(s.thirdMax(nums2))
	// Expecting 1
	println(s.thirdMax(nums3))
}

class Solution {
    fun thirdMax(nums: IntArray): Int {
		var firstMax: Int? = null
		var secondMax: Int? = null
		var thirdMax: Int? = null

		for (num in nums) {
			if (num == firstMax || num == secondMax|| num == thirdMax) {
				continue
			}

			if (firstMax == null || num > firstMax) {
				thirdMax = secondMax
				secondMax = firstMax
				firstMax = num
			} else if (secondMax == null || num > secondMax) {
				thirdMax = secondMax
				secondMax = num
			} else if (thirdMax == null || num > thirdMax) {
				thirdMax = num
			}
		}

		if (thirdMax == null) {
			return firstMax!!
		}

		return thirdMax
    }
}
