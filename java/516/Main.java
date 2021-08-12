public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var s1 = "bbbab";
		final var s2 = "cbbd";
		final var s3 = "";
		final var s4 = "a";

		// Expecting 4
		System.out.println(s.longestPalindromeSubseq(s1));
		// Expecting 2
		System.out.println(s.longestPalindromeSubseq(s2));
		// Expecting 0
		System.out.println(s.longestPalindromeSubseq(s3));
		// Expecting 1
		System.out.println(s.longestPalindromeSubseq(s4));
	}
}

class Solution {
	public int longestPalindromeSubseq(final String s) {
		final var n = s.length();
		if (n == 0) {
			return 0;
		}

		final var dp = new int[n][n];

		for (var i = n - 1; i >= 0; i--) {
			final var chI = s.charAt(i);

			for (var j = i; j < n; j++) {
				final var chJ = s.charAt(j);

				if (j - i < 2) {
					dp[i][j] = chI == chJ ? j - i + 1 : j - i;
					continue;
				}

				if (chI == chJ) {
					dp[i][j] = dp[i + 1][j - 1] + 2;
				} else {
					dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
				}
			}
		}

		return dp[0][n - 1];
	}
}
