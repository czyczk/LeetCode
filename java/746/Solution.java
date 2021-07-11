public class Solution {
	public int minCostClimbingStairs(int[] cost) {
		final var n = cost.length;
		// var dp = new int[n];
		// dp[0] = cost[0];
		// dp[1] = cost[1];
		var dp = cost[1];
		var dpLast = cost[0];

		for (int i = 2; i < n; i++) {
			// dp[i] = Math.min(dp[i - 1], dp[i - 2]) + cost[i];
			final var newDp = Math.min(dp, dpLast) + cost[i];
			dpLast = dp;
			dp = newDp;
		}

		// return Math.min(dp[n - 2], dp[n - 1]);
		return Math.min(dp, dpLast);
	}
}
