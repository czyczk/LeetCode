public class Main {
    public static void main(String[] args) {
        var pairs1 = new int[] { 1, 3, 2, 3, 1 };
        var pairs2 = new int[] { 2, 4, 3, 5, 1 };
        var pairs3 = new int[] { -5, -5 };
        var pairs4 = new int[] { 2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647 };

        var s = new Solution();
        // 2
        System.out.println(s.reversePairs(pairs1));
        // 3
        System.out.println(s.reversePairs(pairs2));
        // 1
        System.out.println(s.reversePairs(pairs3));
        // 0
        System.out.println(s.reversePairs(pairs4));
    }
}