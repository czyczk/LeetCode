public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var p1 = new int[] { 3, 3, 5, 0, 0, 3, 1, 4 };
        final var p2 = new int[] { 1, 2, 3, 4, 5 };
        final var p3 = new int[] { 7, 6, 4, 3, 1 };
        final var p4 = new int[] { 1 };

        // Expecting 6
        System.out.println(s.maxProfit(p1));
        // Expecting 4
        System.out.println(s.maxProfit(p2));
        // Expecting 0
        System.out.println(s.maxProfit(p3));
        // Expecting 0
        System.out.println(s.maxProfit(p4));
    }
}

class Solution {
    public int maxProfit(final int[] prices) {
        final var n = prices.length;
        if (n == 0) {
            return 0;
        }

        var dpBought1 = -prices[0];
        var dpSold1 = 0;
        var dpBought2 = -prices[0];
        var dpSold2 = 0;

        for (var i = 1; i < n; i++) {
            dpBought1 = Math.max(dpBought1, -prices[i]);
            dpSold1 = Math.max(dpSold1, dpBought1 + prices[i]);
            dpBought2 = Math.max(dpBought2, dpSold1 - prices[i]);
            dpSold2 = Math.max(dpSold2, dpBought2 + prices[i]);
        }

        return dpSold2;
    }
}