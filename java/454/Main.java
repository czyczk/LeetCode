public class Main {
    public static void main(String[] args) {
        var A = new int[] { 1, 2 };
        var B = new int[] { -2, -1 };
        var C = new int[] { -1, 2 };
        var D = new int[] { 0, 2 };

        var s = new Solution();
        // Expecting 2
        System.out.println(s.fourSumCount(A, B, C, D));
    }
}