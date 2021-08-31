public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var p1 = new int[] { 1, 3, 2, 8, 4, 9 };
        final var f1 = 2;

        final var p2 = new int[] { 1, 3, 7, 5, 10, 3 };
        final var f2 = 3;

        // Expecting 8
        System.out.println(s.maxProfit(p1, f1));
        // Expecting 6
        System.out.println(s.maxProfit(p2, f2));
    }
}

class Solution {
    public int maxProfit(final int[] prices, final int fee) {
        final var n = prices.length;
        if (n == 0) {
            return 0;
        }

        var dpBought = -prices[0] - fee;
        var dpSold = 0;

        for (var i = 1; i < n; i++) {
            dpBought = Math.max(dpBought, dpSold - prices[i] - fee);
            dpSold = Math.max(dpSold, dpBought + prices[i]);
        }

        return dpSold;
    }
}