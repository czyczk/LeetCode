import java.util.ArrayList;
import java.util.List;

public class Solution {
    public List<List<Integer>> generate(int numRows) {
        var result = new ArrayList<List<Integer>>();
        var shouldEarlyReturn = handleEarlyReturns(numRows, result);
        if (shouldEarlyReturn) {
            return result;
        }

        for (var i = 3; i <= numRows; i++) {
            var row = new ArrayList<Integer>();
            row.add(1);

            for (var j = 1; j < i - 1; j++) {
                row.add(result.get(i - 2).get(j - 1) + result.get(i - 2).get(j));
            }

            row.add(1);
            result.add(row);
        }

        return result;
    }

    private boolean handleEarlyReturns(int numRows, List<List<Integer>> result) {
        if (numRows == 0) {
            return true;
        }

        var row = new ArrayList<Integer>();
        row.add(1);
        result.add(row);

        if (numRows == 1) {
            return true;
        }

        row = new ArrayList<Integer>();
        row.add(1);
        row.add(1);
        result.add(row);

        if (numRows == 2) {
            return true;
        }

        return false;
    }
}