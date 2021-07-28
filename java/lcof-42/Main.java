public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();
        final var n1 = new int[] { -2, 1, -3, 4, -1, 2, 1, -5, 4 };

        // Expecting 6
        System.out.println(s.maxSubArray(n1));
    }

}

class Solution {
    public int maxSubArray(final int[] nums) {
        final var n = nums.length;
        final var dp = new int[n];
        dp[0] = nums[0];

        var ret = dp[0];

        for (int i = 1; i < n; i++) {
            dp[i] = Math.max(nums[i], dp[i - 1] + nums[i]);
            ret = Math.max(ret, dp[i]);
        }

        return ret;
    }
}