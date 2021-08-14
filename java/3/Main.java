public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var s1 = "abcabcbb";
		final var s2 = "bbbbb";
		final var s3 = "pwwkew";
		final var s4 = "";
		final var s5 = "dvdf";

		// Expecting 3
		System.out.println(s.lengthOfLongestSubstring(s1));
		// Expecting 1
		System.out.println(s.lengthOfLongestSubstring(s2));
		// Expecting 3
		System.out.println(s.lengthOfLongestSubstring(s3));
		// Expecting 0
		System.out.println(s.lengthOfLongestSubstring(s4));
		// Expecting 3
		System.out.println(s.lengthOfLongestSubstring(s5));
	}
}

class Solution {
	public int lengthOfLongestSubstring(final String s) {
		if (s.isEmpty()) {
			return 0;
		}

		final var isUsed = new boolean[128];
		var start = 0;
		var end = -1; // inclusive
		var ret = 0;

		for (final var ch : s.toCharArray()) {
			if (isUsed[ch]) {
				while (start <= end && s.charAt(start) != ch) {
					isUsed[s.charAt(start)] = false;
					start++;
				}
				start++;
			} else {
				isUsed[ch] = true;
			}
			end++;
			ret = Math.max(ret, end - start + 1);
		}

		return ret;
	}
}
