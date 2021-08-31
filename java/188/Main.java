public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var k1 = 2;
        final var p1 = new int[] { 2, 4, 1 };

        final var k2 = 2;
        final var p2 = new int[] { 3, 2, 6, 5, 0, 3 };

        final var k3 = 0;
        final var p3 = new int[] { 3, 2, 6, 5, 0, 3 };

        // Expecting 2
        System.out.println(s.maxProfit(k1, p1));
        // Expecting 7
        System.out.println(s.maxProfit(k2, p2));
        // Expecting 0
        System.out.println(s.maxProfit(k3, p3));
    }
}

class Solution {
    public int maxProfit(final int k, final int[] prices) {
        if (k == 0) {
            return 0;
        }

        final var n = prices.length;
        if (n == 0) {
            return 0;
        }

        final var dpBought = new int[k];
        final var dpSold = new int[k];
        for (var i = 0; i < k; i++) {
            dpBought[i] = -prices[0];
        }

        for (var i = 1; i < n; i++) {
            dpBought[0] = Math.max(dpBought[0], -prices[i]);
            dpSold[0] = Math.max(dpSold[0], dpBought[0] + prices[i]);
            for (var j = 1; j < k; j++) {
                dpBought[j] = Math.max(dpBought[j], dpSold[j - 1] - prices[i]);
                dpSold[j] = Math.max(dpSold[j], dpBought[j] + prices[i]);
            }
        }

        return dpSold[k - 1];
    }
}