import java.util.Arrays;

public class Solution {
    public int largestPerimeter(int[] A) {
        if (A == null || A.length < 3) {
            return 0;
        }

        Arrays.sort(A);

        var maxPerimeter = 0;

        for (var i = A.length - 1; i >= 2; i--) {
            if (A[i - 2] + A[i - 1] > A[i]) {
                maxPerimeter = Math.max(A[i - 2] + A[i - 1] + A[i], maxPerimeter);
            }
        }

        return maxPerimeter;
    }
}
