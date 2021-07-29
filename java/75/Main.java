import java.util.Arrays;

public class Main {
	public static void main(String[] args) {
		var s = new Solution();

		var n1 = new int[] { 2, 0, 2, 1, 1, 0 };
		var n2 = new int[] { 2, 0, 1 };
		var n3 = new int[] { 0 };
		var n4 = new int[] { 1 };
		s.sortColors(n1);
		s.sortColors(n2);
		s.sortColors(n3);
		s.sortColors(n4);

		// Expecting [0, 0, 1, 1, 2, 2]
		System.out.println(Arrays.toString(n1));
		// Expecting [0, 1, 2]
		System.out.println(Arrays.toString(n2));
		// Expecting [0]
		System.out.println(Arrays.toString(n3));
		// Expecting [1]
		System.out.println(Arrays.toString(n4));
	}
}

class Solution {
	public void sortColors(int[] nums) {
		int p0 = 0;
		int p1 = 0;
		int i = 0;
		var n = nums.length;

		while (i < n) {
			if (nums[i] == 0) {
				var temp = nums[i];
				nums[i] = nums[p0];
				nums[p0] = temp;

				if (p1 > p0) {
					temp = nums[i];
					nums[i] = nums[p1];
					nums[p1] = temp;
				}

				p0++;
				p1++;
			} else if (nums[i] == 1) {
				var temp = nums[i];
				nums[i] = nums[p1];
				nums[p1] = temp;
				p1++;
			}

			i++;
		}
	}
}
