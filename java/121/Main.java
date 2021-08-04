public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var p1 = new int[] { 7, 1, 5, 3, 6, 4 };
        final var p2 = new int[] { 7, 6, 4, 3, 1 };

        // Expecting 5
        System.out.println(s.maxProfit(p1));
        // Expecting 0
        System.out.println(s.maxProfit(p2));
    }
}

class Solution {
    public int maxProfit(final int[] prices) {
        final var n = prices.length;
        if (n == 0) {
            return 0;
        }

        var dpBought = -prices[0];
        var dpSold = 0;

        for (int i = 1; i < n; i++) {
            dpBought = Math.max(dpBought, -prices[i]);
            dpSold = Math.max(dpSold, dpBought + prices[i]);
        }

        return dpSold;
    }
}