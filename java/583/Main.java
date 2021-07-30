public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final String w11 = "sea", w12 = "eat";
        final String w21 = "leetcode", w22 = "etco";

        // Expecting 2
        System.out.println(s.minDistance(w11, w12));
        // Expecting 4
        System.out.println(s.minDistance(w21, w22));
    }
}

class Solution {
    public int minDistance(final String word1, final String word2) {
        final var len1 = word1.length();
        final var len2 = word2.length();
        final var dp = new int[len1 + 1][len2 + 1];

        for (int i = 1; i <= len1; i++) {
            dp[i][0] = i;
        }

        for (int j = 1; j <= len2; j++) {
            dp[0][j] = j;
        }

        for (int i = 1; i <= len1; i++) {
            for (int j = 1; j <= len2; j++) {
                if (word1.charAt(i - 1) == word2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = Math.min(dp[i][j - 1] + 1, dp[i - 1][j] + 1);
                }
            }
        }

        return dp[len1][len2];
    }
}