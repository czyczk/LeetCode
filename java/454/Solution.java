import java.util.HashMap;

public class Solution {
    public int fourSumCount(int[] A, int[] B, int[] C, int[] D) {
        var numSolutions = 0;
        final var abMap = constructSumCountMap(A, B);

        for (final var numFromC : C) {
            for (final var numFromD : D) {
                var sumNeg = -(numFromC + numFromD);

                numSolutions += abMap.getOrDefault(sumNeg, 0);
            }
        }

        return numSolutions;
    }

    private HashMap<Integer, Integer> constructSumCountMap(int[] a, int[] b) {
        final var sumCountMap = new HashMap<Integer, Integer>();

        for (final var numFromA : a) {
            for (final var numFromB : b) {
                var sum = numFromA + numFromB;

                sumCountMap.put(sum, sumCountMap.getOrDefault(sum, 0) + 1);
            }
        }

        return sumCountMap;
    }
}
