import java.util.HashMap;

public class SolutionGreedy {
    public boolean isPossible(int[] nums) {
        var cnt = new HashMap<Integer, Integer>();
        var numsSubSeq = new HashMap<Integer, Integer>();

        for (var num : nums) {
            cnt.put(num, cnt.getOrDefault(num, 0) + 1);
        }

        for (var num : nums) {
            if (cnt.get(num) == 0) {
                continue;
            }

            cnt.put(num, cnt.get(num) - 1);

            var numSubSeq = numsSubSeq.getOrDefault(num - 1, 0);
            if (numSubSeq > 0) {
                numsSubSeq.put(num - 1, numSubSeq - 1);
                numsSubSeq.put(num, numsSubSeq.getOrDefault(num, 0) + 1);
            } else {
                var cntNumPlusOne = cnt.getOrDefault(num + 1, 0);
                if (cntNumPlusOne == 0) {
                    return false;
                }

                var cntNumPlusTwo = cnt.getOrDefault(num + 2, 0);
                if (cntNumPlusTwo == 0) {
                    return false;
                }

                cnt.put(num + 1, cntNumPlusOne - 1);
                cnt.put(num + 2, cntNumPlusTwo - 1);
                numsSubSeq.put(num + 2, numsSubSeq.getOrDefault(num + 2, 0) + 1);
            }
        }

        return true;
    }
}