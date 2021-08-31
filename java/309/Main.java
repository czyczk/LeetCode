public class Main {
    public static void main(String[] args) {
        var s = new Solution();

        var p1 = new int[] { 1, 2, 3, 0, 2 };

        // Expecting 3
        System.out.println(s.maxProfit(p1));
    }
}

class Solution {
    public int maxProfit(int[] prices) {
        var n = prices.length;
        if (n == 0) {
            return 0;
        }

        var dp0 = -prices[0]; // Bought
        var dp1 = 0; // Sold (after cd)
        var dp2 = 0; // CDing
        var dp3 = 0; // Sold today

        for (var i = 1; i < n; i++) {
            dp0 = Math.max(dp0, Math.max(dp1, dp2) - prices[i]);
            dp1 = Math.max(dp1, dp2);
            dp2 = Math.max(dp2, dp3);
            dp3 = Math.max(dp3, dp0 + prices[i]);
        }

        return dp3;
    }
}