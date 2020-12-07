public class Main {
    public static void main(String[] args) {
        var matrix = new int[][] { new int[] { 0, 0, 1, 1 }, new int[] { 1, 0, 1, 0 }, new int[] { 1, 1, 0, 0 } };

        var s = new Solution();
        // Expecting 39
        System.out.println(s.matrixScore(matrix));
    }
}