public class Solution {
	public int strStr(String haystack, String needle) {
		char[] haystackChars = haystack.toCharArray();
		char[] needleChars = needle.toCharArray();

		int m = haystackChars.length;
		int n = needleChars.length;

		if (n == 0) {
			return 0;
		}

		if (m == 0) {
			return -1;
		}

		int[] next = buildNext(needleChars);

		int j = 0;
		for (int i = 0; i < m; i++) {
			while (j > 0 && haystackChars[i] != needleChars[j]) {
				j = next[j - 1];
			}

			if (haystackChars[i] == needleChars[j]) {
				j++;
				if (j >= n) {
					return i - n + 1;
				}
			}
		}

		return -1;
	}

	private int[] buildNext(char[] needle) {
		int n = needle.length;
		int[] next = new int[n];

		int j = 0;
		for (int i = 1; i < n; i++) {
			while (j > 0 && needle[i] != needle[j]) {
				j = next[j - 1];
			}

			if (needle[i] == needle[j]) {
				j++;
			}

			next[i] = j;
		}

		return next;
	}
}
