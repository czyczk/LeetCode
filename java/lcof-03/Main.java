public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var n1 = new int[] { 2, 3, 1, 0, 2, 5, 3 };
		final var n2 = new int[] { 3, 4, 2, 0, 0, 1 };

		// Expecting 2 or 3
		System.out.println(s.findRepeatNumber(n1));
		// Expecting 0
		System.out.println(s.findRepeatNumber(n2));
	}
}

class Solution {
	public int findRepeatNumber(final int[] nums) {
		final var n = nums.length;
		var i = 0;

		while (i < n) {
			if (nums[i] == i) {
				i++;
				continue;
			}

			if (nums[i] == nums[nums[i]]) {
				return nums[i];
			}

			final var temp = nums[nums[i]];
			nums[nums[i]] = nums[i];
			nums[i] = temp;
		}

		return -1;
	}
}
